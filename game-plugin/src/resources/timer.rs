use bevy::prelude::*;
use common::consts::SPACE_KEY_HOLD_DURATION;

#[derive(Resource)]
pub struct SimulationTimer(pub Timer);

impl SimulationTimer {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct SpaceKeyTimer(pub Timer);

impl SpaceKeyTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(SPACE_KEY_HOLD_DURATION, TimerMode::Once))
    }
}
