use bevy::prelude::*;

#[derive(Resource)]
pub struct SpaceKeyTimer(pub Timer);

impl SpaceKeyTimer {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}
