use bevy::prelude::States;

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug, States)]
pub enum SimulationState {
    #[default]
    Paused,
    Simulating,
}
