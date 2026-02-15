//! ボタンクリック・ホバーのアクションハンドラ

use bevy::prelude::*;
use common::consts::{BG_BUTTON, BG_BUTTON_HOVER};

use crate::components::action::GameButtonAction;
use crate::events::{
    GenerationResetEvent, PlayAudioEvent, ProgressGenerationEvent, WorldClearEvent,
};
use crate::resources::interaction::GridVisible;
use crate::states::SimulationState;

/// シミュレーション開始/停止トグルボタンのクリックハンドラ
pub fn handle_toggle_simulation(
    _click: On<Pointer<Click>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
) {
    match simulation_state.get() {
        SimulationState::Paused => simulation_next_state.set(SimulationState::Simulating),
        SimulationState::Simulating => simulation_next_state.set(SimulationState::Paused),
    }
}

/// シミュレーション状態に応じてトグルボタンのテキストを更新するシステム
pub fn update_toggle_button_text(
    simulation_state: Res<State<SimulationState>>,
    query_button: Query<(&GameButtonAction, &Children)>,
    mut query_text: Query<&mut Text>,
) {
    let label = match simulation_state.get() {
        SimulationState::Paused => "Start",
        SimulationState::Simulating => "Stop",
    };
    for (action, children) in query_button.iter() {
        if matches!(action, GameButtonAction::ToggleSimulation) {
            for child in children.iter() {
                if let Ok(mut text) = query_text.get_mut(child) {
                    **text = label.to_string();
                }
            }
        }
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

/// ボタンホバー時のハンドラ: 背景色を変更し効果音を再生する
pub fn handle_over(
    over: On<Pointer<Over>>,
    mut query: Query<&mut BackgroundColor>,
    mut events: MessageWriter<PlayAudioEvent>,
) {
    if let Ok(mut background_color) = query.get_mut(over.entity) {
        background_color.0 = BG_BUTTON_HOVER;
        events.write(PlayAudioEvent);
    }
}

/// ボタンホバー終了時のハンドラ: 背景色を元に戻す
pub fn handle_out(out: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor>) {
    if let Ok(mut background_color) = query.get_mut(out.entity) {
        background_color.0 = BG_BUTTON;
    }
}

/// Gridボタンのクリックハンドラ: グリッドライン表示を切り替える
pub fn handle_grid_toggle(_click: On<Pointer<Click>>, mut grid_visible: ResMut<GridVisible>) {
    grid_visible.0 = !grid_visible.0;
}
