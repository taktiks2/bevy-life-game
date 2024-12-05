use bevy::prelude::*;

use common::states::GameState;

use crate::events::ProgressGenerationEvent;
use crate::states::SimulationState;

pub fn game_input_keyboard_handling(
    keys: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut progress_generation_event_writer: EventWriter<ProgressGenerationEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        progress_generation_event_writer.send(ProgressGenerationEvent);
    }
    if keys.pressed(KeyCode::Space) && *simulation_state.get() == SimulationState::Paused {
        simulation_next_state.set(SimulationState::Simulating);
    }
    if keys.just_released(KeyCode::Space) && *simulation_state.get() == SimulationState::Simulating
    {
        simulation_next_state.set(SimulationState::Paused);
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        game_next_state.set(GameState::Menu);
    }
}
