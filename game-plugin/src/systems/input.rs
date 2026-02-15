//! キーボード・マウス入力のハンドリング
//!
//! スペースキーのステートマシン（短押し/長押し判定）、
//! WASD/QEキーによるカメラ操作、マウスホイールズーム、
//! 左クリックドラッグパンを処理する。

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use common::consts::{
    CAMERA_PAN_SPEED, CAMERA_SCALE_STEP, DRAG_THRESHOLD, MAX_CAMERA_SCALE, MIN_CAMERA_SCALE,
    MOUSE_WHEEL_ZOOM_SENSITIVITY, calc_viewport_sizes,
};
use common::states::GameState;

use crate::WorldCamera;
use crate::events::ProgressGenerationEvent;
use crate::resources::interaction::DragState;
use crate::resources::timer::SpaceKeyTimer;
use crate::states::SimulationState;
use crate::systems::coordinate::is_cursor_over_world_viewport;

/// スペースキーの入力状態をまとめた構造体
#[derive(Debug)]
pub(crate) struct SpaceKeyInput {
    pub just_pressed: bool,
    pub pressed: bool,
    pub just_released: bool,
    pub hold_timer_finished: bool,
    pub is_paused: bool,
}

/// スペースキーの入力状態から決定されるアクション
#[derive(Debug, PartialEq)]
pub(crate) enum SpaceKeyAction {
    /// 短押し: 1世代進める
    StepOnce,
    /// 長押し + Paused: 自動シミュレーション開始
    StartSimulating,
    /// 離す + Simulating: シミュレーション停止
    StopSimulating,
    /// 離す + Paused: タイマーリセットのみ
    ResetTimer,
    /// 何もしない
    None,
}

/// スペースキーの入力状態からアクションを決定する純粋関数
///
/// 優先順位: just_pressed > 長押し完了 > just_released > None
pub(crate) fn resolve_space_key_action(input: &SpaceKeyInput) -> SpaceKeyAction {
    if input.just_pressed {
        return SpaceKeyAction::StepOnce;
    }
    if input.pressed && input.hold_timer_finished && input.is_paused {
        return SpaceKeyAction::StartSimulating;
    }
    if input.just_released {
        if !input.is_paused {
            return SpaceKeyAction::StopSimulating;
        }
        return SpaceKeyAction::ResetTimer;
    }
    SpaceKeyAction::None
}

/// キーボード入力を処理するシステム
///
/// - スペースキー: 短押しで1世代進める、長押しで自動シミュレーション開始/停止
/// - Escapeキー: メニュー画面に遷移
pub fn game_input_keyboard_handling(
    keys: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut progress_generation_event_writer: MessageWriter<ProgressGenerationEvent>,
    time: Res<Time>,
    mut space_key_timer: ResMut<SpaceKeyTimer>,
) {
    let space_input = SpaceKeyInput {
        just_pressed: keys.just_pressed(KeyCode::Space),
        pressed: keys.pressed(KeyCode::Space),
        just_released: keys.just_released(KeyCode::Space),
        hold_timer_finished: keys.pressed(KeyCode::Space)
            && space_key_timer.0.tick(time.delta()).is_finished(),
        is_paused: *simulation_state.get() == SimulationState::Paused,
    };

    match resolve_space_key_action(&space_input) {
        SpaceKeyAction::StepOnce => {
            progress_generation_event_writer.write(ProgressGenerationEvent);
        }
        SpaceKeyAction::StartSimulating => {
            simulation_next_state.set(SimulationState::Simulating);
        }
        SpaceKeyAction::StopSimulating => {
            space_key_timer.0.reset();
            simulation_next_state.set(SimulationState::Paused);
        }
        SpaceKeyAction::ResetTimer => {
            space_key_timer.0.reset();
        }
        SpaceKeyAction::None => {}
    }

    if keys.just_pressed(KeyCode::Escape) {
        game_next_state.set(GameState::Menu);
    }
}

/// WASD/QEキーによるカメラ操作システム
///
/// - W/A/S/D: カメラのパン（上下左右移動）
/// - Q: ズームアウト、E: ズームイン
pub fn game_input_zoom_handling(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<WorldCamera>>,
) {
    let Ok((mut transform, mut projection)) = camera_query.single_mut() else {
        return;
    };
    if keys.just_pressed(KeyCode::KeyW) {
        transform.translation.y += CAMERA_PAN_SPEED;
    }
    if keys.just_pressed(KeyCode::KeyS) {
        transform.translation.y -= CAMERA_PAN_SPEED;
    }
    if keys.just_pressed(KeyCode::KeyA) {
        transform.translation.x -= CAMERA_PAN_SPEED;
    }
    if keys.just_pressed(KeyCode::KeyD) {
        transform.translation.x += CAMERA_PAN_SPEED;
    }
    if let Projection::Orthographic(ref mut ortho) = *projection {
        if keys.just_pressed(KeyCode::KeyQ) {
            ortho.scale = (ortho.scale + CAMERA_SCALE_STEP).min(MAX_CAMERA_SCALE);
        }
        if keys.just_pressed(KeyCode::KeyE) {
            ortho.scale = (ortho.scale - CAMERA_SCALE_STEP).max(MIN_CAMERA_SCALE);
        }
    }
}

/// マウスホイールのスクロール量からズームスケールを計算する純粋関数
///
/// スクロール上（正値）でズームイン（スケール減少）、
/// スクロール下（負値）でズームアウト（スケール増加）。
pub(crate) fn calc_zoom_scale(current_scale: f32, scroll_y: f32) -> f32 {
    let delta = -scroll_y * MOUSE_WHEEL_ZOOM_SENSITIVITY;
    (current_scale + delta).clamp(MIN_CAMERA_SCALE, MAX_CAMERA_SCALE)
}

/// カーソルの移動量がドラッグ閾値を超えているか判定する純粋関数
pub(crate) fn exceeds_drag_threshold(start_pos: Vec2, current_pos: Vec2) -> bool {
    start_pos.distance(current_pos) > DRAG_THRESHOLD
}

/// マウスホイールによるズーム操作システム
///
/// ワールドビューポート上でのみ動作する。
pub fn mouse_wheel_zoom(
    mut scroll_events: MessageReader<MouseWheel>,
    windows: Query<&Window>,
    mut camera_query: Query<&mut Projection, With<WorldCamera>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let scale_factor = window.resolution.scale_factor();
    let sizes = calc_viewport_sizes(window.physical_width(), window.physical_height());
    if !is_cursor_over_world_viewport(cursor_pos, scale_factor, sizes.main_height) {
        return;
    }
    let total_scroll: f32 = scroll_events.read().map(|e| e.y).sum();
    if total_scroll == 0.0 {
        return;
    }
    let Ok(mut projection) = camera_query.single_mut() else {
        return;
    };
    if let Projection::Orthographic(ref mut ortho) = *projection {
        ortho.scale = calc_zoom_scale(ortho.scale, total_scroll);
    }
}

/// 左クリックドラッグによるカメラパン操作システム
///
/// クリック開始位置からの移動量が閾値を超えるとドラッグモードに入り、
/// カーソル移動量に応じてカメラを移動する。
pub fn mouse_drag_pan(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut camera_query: Query<(&mut Transform, &Projection), With<WorldCamera>>,
    mut drag_state: ResMut<DragState>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    // 左クリック開始
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            let scale_factor = window.resolution.scale_factor();
            let sizes = calc_viewport_sizes(window.physical_width(), window.physical_height());
            if is_cursor_over_world_viewport(cursor_pos, scale_factor, sizes.main_height) {
                drag_state.start_pos = Some(cursor_pos);
                drag_state.last_pos = Some(cursor_pos);
                drag_state.is_dragging = false;
            }
        }
        return;
    }

    // 左クリック中のドラッグ処理
    if mouse.pressed(MouseButton::Left) {
        if let (Some(start_pos), Some(last_pos)) = (drag_state.start_pos, drag_state.last_pos) {
            if let Some(current_pos) = window.cursor_position() {
                // 閾値超えでドラッグモード開始
                if !drag_state.is_dragging && exceeds_drag_threshold(start_pos, current_pos) {
                    drag_state.is_dragging = true;
                }
                if drag_state.is_dragging {
                    let delta = current_pos - last_pos;
                    let Ok((mut transform, projection)) = camera_query.single_mut() else {
                        return;
                    };
                    // スクリーンピクセルをワールド座標に変換（スケール考慮）
                    let scale = if let Projection::Orthographic(ref ortho) = *projection {
                        ortho.scale
                    } else {
                        1.0
                    };
                    // スクリーン座標系は下向きY、ワールド座標系は上向きYなので反転
                    transform.translation.x -= delta.x * scale;
                    transform.translation.y += delta.y * scale;
                }
                drag_state.last_pos = Some(current_pos);
            }
        }
        return;
    }

    // 左クリック解放
    if mouse.just_released(MouseButton::Left) {
        drag_state.start_pos = None;
        drag_state.last_pos = None;
        drag_state.is_dragging = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(
        just_pressed: bool,
        pressed: bool,
        just_released: bool,
        hold_timer_finished: bool,
        is_paused: bool,
    ) -> SpaceKeyInput {
        SpaceKeyInput {
            just_pressed,
            pressed,
            just_released,
            hold_timer_finished,
            is_paused,
        }
    }

    #[test]
    fn just_pressed_returns_step_once() {
        let result = resolve_space_key_action(&input(true, true, false, false, true));
        assert_eq!(result, SpaceKeyAction::StepOnce);
    }

    #[test]
    fn long_press_paused_returns_start_simulating() {
        let result = resolve_space_key_action(&input(false, true, false, true, true));
        assert_eq!(result, SpaceKeyAction::StartSimulating);
    }

    #[test]
    fn long_press_simulating_returns_none() {
        // 既にSimulating中の長押しは何もしない
        let result = resolve_space_key_action(&input(false, true, false, true, false));
        assert_eq!(result, SpaceKeyAction::None);
    }

    #[test]
    fn released_while_simulating_returns_stop() {
        let result = resolve_space_key_action(&input(false, false, true, false, false));
        assert_eq!(result, SpaceKeyAction::StopSimulating);
    }

    #[test]
    fn released_while_paused_returns_reset_timer() {
        let result = resolve_space_key_action(&input(false, false, true, false, true));
        assert_eq!(result, SpaceKeyAction::ResetTimer);
    }

    #[test]
    fn no_input_returns_none() {
        let result = resolve_space_key_action(&input(false, false, false, false, true));
        assert_eq!(result, SpaceKeyAction::None);
    }

    #[test]
    fn pressed_without_timer_finished_returns_none() {
        // 長押し中だがタイマー未完了
        let result = resolve_space_key_action(&input(false, true, false, false, true));
        assert_eq!(result, SpaceKeyAction::None);
    }

    // --- マウスホイールズーム計算テスト ---

    #[test]
    fn calc_zoom_scroll_up_zooms_in() {
        // スクロール上（正値）→ スケール減少（ズームイン）
        let result = calc_zoom_scale(0.5, 1.0);
        assert!(result < 0.5);
    }

    #[test]
    fn calc_zoom_scroll_down_zooms_out() {
        // スクロール下（負値）→ スケール増加（ズームアウト）
        let result = calc_zoom_scale(0.15, -1.0);
        assert!(result > 0.15);
    }

    #[test]
    fn calc_zoom_clamps_to_min() {
        // 大きなスクロールでも最小値を下回らない
        let result = calc_zoom_scale(0.15, 100.0);
        assert_eq!(result, MIN_CAMERA_SCALE);
    }

    #[test]
    fn calc_zoom_clamps_to_max() {
        // 大きなスクロールでも最大値を上回らない
        let result = calc_zoom_scale(0.9, -100.0);
        assert_eq!(result, MAX_CAMERA_SCALE);
    }

    #[test]
    fn calc_zoom_zero_scroll_no_change() {
        let result = calc_zoom_scale(0.15, 0.0);
        assert_eq!(result, 0.15);
    }

    // --- ドラッグ閾値判定テスト ---

    #[test]
    fn drag_threshold_not_exceeded_small_movement() {
        let start = Vec2::new(100.0, 100.0);
        let current = Vec2::new(102.0, 103.0);
        assert!(!exceeds_drag_threshold(start, current));
    }

    #[test]
    fn drag_threshold_exceeded_large_movement() {
        let start = Vec2::new(100.0, 100.0);
        let current = Vec2::new(110.0, 100.0);
        assert!(exceeds_drag_threshold(start, current));
    }

    #[test]
    fn drag_threshold_exact_boundary_not_exceeded() {
        // ちょうど閾値の距離は超えていない（> ではなく >）
        let start = Vec2::ZERO;
        let current = Vec2::new(DRAG_THRESHOLD, 0.0);
        assert!(!exceeds_drag_threshold(start, current));
    }

    #[test]
    fn drag_threshold_just_over_boundary() {
        let start = Vec2::ZERO;
        let current = Vec2::new(DRAG_THRESHOLD + 0.01, 0.0);
        assert!(exceeds_drag_threshold(start, current));
    }
}
