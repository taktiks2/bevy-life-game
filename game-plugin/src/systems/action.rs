use bevy::prelude::*;

use crate::components::action::GameButtonAction;
use crate::events::{GenerationResetEvent, ProgressGenerationEvent, WorldClearEvent};
use crate::states::SimulationState;
use crate::WorldCamera;

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
    mut query_camera: Query<&mut OrthographicProjection, With<WorldCamera>>,
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
                GameButtonAction::SpeedDown => {
                    println!("SpeedDown");
                }
                GameButtonAction::SpeedUp => {
                    println!("SpeedUp");
                }
                GameButtonAction::ZoomDown => {
                    for mut camera in query_camera.iter_mut() {
                        camera.scale = (camera.scale + 0.1).min(1.0);
                    }
                }
                GameButtonAction::ZoomUp => {
                    for mut camera in query_camera.iter_mut() {
                        camera.scale = (camera.scale - 0.1).max(0.1);
                    }
                }
            }
        }
    }
}
