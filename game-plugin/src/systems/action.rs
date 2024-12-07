use bevy::prelude::*;

use crate::components::action::GameButtonAction;
use crate::events::{GenerationResetEvent, ProgressGenerationEvent, WorldClearEvent};
use crate::states::SimulationState;

#[allow(clippy::type_complexity)]
pub fn game_action(
    interaction_query: Query<
        (&Interaction, &GameButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut progress_generation_event_writer: EventWriter<ProgressGenerationEvent>,
    mut generation_reset_event_writer: EventWriter<GenerationResetEvent>,
    mut world_clear_event_writer: EventWriter<WorldClearEvent>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                GameButtonAction::Start => {
                    if *simulation_state.get() == SimulationState::Paused {
                        simulation_next_state.set(SimulationState::Simulating);
                    }
                }
                GameButtonAction::Stop => {
                    if *simulation_state.get() == SimulationState::Simulating {
                        simulation_next_state.set(SimulationState::Paused);
                    }
                }
                GameButtonAction::Next => {
                    progress_generation_event_writer.send(ProgressGenerationEvent);
                }
                GameButtonAction::Reset => {
                    generation_reset_event_writer.send(GenerationResetEvent);
                }
                GameButtonAction::Clear => {
                    world_clear_event_writer.send(WorldClearEvent);
                }
            }
        }
    }
}
