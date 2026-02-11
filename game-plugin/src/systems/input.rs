use bevy::prelude::*;
use common::consts::{CAMERA_PAN_SPEED, CAMERA_SCALE_STEP, MAX_CAMERA_SCALE, MIN_CAMERA_SCALE};
use common::states::GameState;

use crate::events::ProgressGenerationEvent;
use crate::resources::timer::SpaceKeyTimer;
use crate::states::SimulationState;
use crate::WorldCamera;

#[derive(Debug)]
pub(crate) struct SpaceKeyInput {
    pub just_pressed: bool,
    pub pressed: bool,
    pub just_released: bool,
    pub hold_timer_finished: bool,
    pub is_paused: bool,
}

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
}
