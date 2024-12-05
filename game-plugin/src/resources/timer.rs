use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationTimer(pub Timer);

impl SimulationTimer {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}
