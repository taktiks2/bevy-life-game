//! ボタンクリック・ホバーのアクションハンドラ

use bevy::{color::palettes::css::NAVY, prelude::*};
use common::consts::{
    CAMERA_SCALE_STEP, MAX_CAMERA_SCALE, MAX_TICK_INTERVAL, MIN_CAMERA_SCALE, MIN_TICK_INTERVAL,
    TICK_INTERVAL_STEP,
};

use crate::events::{
    GenerationResetEvent, PlayAudioEvent, ProgressGenerationEvent, WorldClearEvent,
};
use crate::resources::timer::SimulationTimer;
use crate::states::SimulationState;
use crate::WorldCamera;

/// Startボタンのクリックハンドラ: シミュレーションを開始する
pub fn handle_start(
    _click: On<Pointer<Click>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
) {
    if *simulation_state.get() == SimulationState::Paused {
        simulation_next_state.set(SimulationState::Simulating);
    }
}

/// Stopボタンのクリックハンドラ: シミュレーションを一時停止する
pub fn handle_stop(
    _click: On<Pointer<Click>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
) {
    if *simulation_state.get() == SimulationState::Simulating {
        simulation_next_state.set(SimulationState::Paused);
    }
}

/// Nextボタンのクリックハンドラ: 1世代進める
pub fn handle_next(
    _click: On<Pointer<Click>>,
    mut progress_generation_event_writer: MessageWriter<ProgressGenerationEvent>,
) {
    progress_generation_event_writer.write(ProgressGenerationEvent);
}

/// Resetボタンのクリックハンドラ: 初期パターンに戻す
pub fn handle_reset(
    _click: On<Pointer<Click>>,
    mut generation_reset_event_writer: MessageWriter<GenerationResetEvent>,
) {
    generation_reset_event_writer.write(GenerationResetEvent);
}

/// Clearボタンのクリックハンドラ: 全セルをクリアする
pub fn handle_clear(
    _click: On<Pointer<Click>>,
    mut world_clear_event_writer: MessageWriter<WorldClearEvent>,
) {
    world_clear_event_writer.write(WorldClearEvent);
}

/// 速度ダウンボタンのクリックハンドラ: ティック間隔を延長する
pub fn handle_speed_down(
    _click: On<Pointer<Click>>,
    mut simulation_timer: ResMut<SimulationTimer>,
) {
    let current_duration = simulation_timer.0.duration().as_secs_f32();
    let new_duration = (current_duration + TICK_INTERVAL_STEP).min(MAX_TICK_INTERVAL);
    simulation_timer
        .0
        .set_duration(std::time::Duration::from_secs_f32(new_duration));
}

/// 速度アップボタンのクリックハンドラ: ティック間隔を短縮する
pub fn handle_speed_up(
    _click: On<Pointer<Click>>,
    mut simulation_timer: ResMut<SimulationTimer>,
) {
    let current_duration = simulation_timer.0.duration().as_secs_f32();
    let new_duration = (current_duration - TICK_INTERVAL_STEP).max(MIN_TICK_INTERVAL);
    simulation_timer
        .0
        .set_duration(std::time::Duration::from_secs_f32(new_duration));
}

/// ズームアウトボタンのクリックハンドラ: カメラスケールを拡大する
pub fn handle_zoom_down(
    _click: On<Pointer<Click>>,
    mut query_camera: Query<&mut Projection, With<WorldCamera>>,
) {
    for mut projection in query_camera.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = (ortho.scale + CAMERA_SCALE_STEP).min(MAX_CAMERA_SCALE);
        }
    }
}

/// ズームインボタンのクリックハンドラ: カメラスケールを縮小する
pub fn handle_zoom_up(
    _click: On<Pointer<Click>>,
    mut query_camera: Query<&mut Projection, With<WorldCamera>>,
) {
    for mut projection in query_camera.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = (ortho.scale - CAMERA_SCALE_STEP).max(MIN_CAMERA_SCALE);
        }
    }
}

/// ボタンホバー時のハンドラ: 背景色を変更し効果音を再生する
pub fn handle_over(
    over: On<Pointer<Over>>,
    mut query: Query<&mut BackgroundColor>,
    mut events: MessageWriter<PlayAudioEvent>,
) {
    if let Ok(mut background_color) = query.get_mut(over.entity) {
        background_color.0 = NAVY.into();
        events.write(PlayAudioEvent);
    }
}

/// ボタンホバー終了時のハンドラ: 背景色を元に戻す
pub fn handle_out(out: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor>) {
    if let Ok(mut background_color) = query.get_mut(out.entity) {
        background_color.0 = Color::BLACK;
    }
}
