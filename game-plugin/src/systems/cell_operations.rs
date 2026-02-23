//! セル状態の更新・イベントハンドリング
//!
//! 世代進行・リセット・クリアのイベントを処理する。

use bevy::prelude::*;

use crate::components::screen::GenerationText;
use crate::events::{GenerationResetEvent, ProgressGenerationEvent, WorldClearEvent};
use crate::resources::{timer::SimulationTimer, world::World};

/// 世代カウンターのUI表示を更新するシステム
pub fn update_generation(world: Res<World>, mut query: Query<&mut TextSpan, With<GenerationText>>) {
    if let Ok(mut span) = query.single_mut() {
        span.0 = world.generation_count.to_string();
    }
}

/// シミュレーションタイマーに基づき世代進行イベントを発火するシステム
///
/// `SimulationState::Simulating` の時のみ動作する。
pub fn progress_generation_trigger(
    time: Res<Time>,
    mut simulation_timer: ResMut<SimulationTimer>,
    mut progress_generation_event_writer: MessageWriter<ProgressGenerationEvent>,
) {
    if simulation_timer.0.tick(time.delta()).is_finished() {
        progress_generation_event_writer.write(ProgressGenerationEvent);
    }
}

/// `ProgressGenerationEvent` を受け取りワールドの世代を進めるシステム
pub fn progress_generation(
    mut world: ResMut<World>,
    mut progress_generation_event_reader: MessageReader<ProgressGenerationEvent>,
) {
    for _ in progress_generation_event_reader.read() {
        world.progress_generation()
    }
}

/// `GenerationResetEvent` を受け取りワールドを初期パターンにリセットするシステム
pub fn reset_generation(
    mut world: ResMut<World>,
    mut generation_reset_event_reader: MessageReader<GenerationResetEvent>,
) {
    for _ in generation_reset_event_reader.read() {
        world.reset();
    }
}

/// `WorldClearEvent` を受け取り全セルをクリアするシステム
pub fn world_clear(
    mut world: ResMut<World>,
    mut world_clear_event_reader: MessageReader<WorldClearEvent>,
) {
    for _ in world_clear_event_reader.read() {
        world.clear();
    }
}
