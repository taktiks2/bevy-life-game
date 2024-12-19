use bevy::prelude::Component;

#[derive(Component)]
pub enum GameButtonAction {
    Start,
    Stop,
    Next,
    Reset,
    Clear,
    SpeedUp,
    SpeedDown,
    ZoomUp,
    ZoomDown,
}
