use bevy::prelude::*;

use common::states::GameState;

use crate::events::ProgressGenerationEvent;
use crate::resources::timer::SpaceKeyTimer;
use crate::states::SimulationState;

pub fn game_input_keyboard_handling(
    keys: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut progress_generation_event_writer: EventWriter<ProgressGenerationEvent>,
    time: Res<Time>,
    mut space_key_timer: ResMut<SpaceKeyTimer>,
) {
    if keys.just_pressed(KeyCode::Space) {
        progress_generation_event_writer.send(ProgressGenerationEvent);
    }
    if keys.pressed(KeyCode::Space)
        && space_key_timer.0.tick(time.delta()).finished()
        && *simulation_state.get() == SimulationState::Paused
    {
        // NOTE: スペースキーを長押しして、PausedのときにSimulating開始
        simulation_next_state.set(SimulationState::Simulating);
    }
    if keys.just_released(KeyCode::Space) {
        // NOTE: スペースキーを離したら必ずタイマーをリセット
        space_key_timer.0.reset();
        if *simulation_state.get() == SimulationState::Simulating {
            // NOTE: Simulating中だったら、Pausedにする
            simulation_next_state.set(SimulationState::Paused);
        }
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        game_next_state.set(GameState::Menu);
    }
}
