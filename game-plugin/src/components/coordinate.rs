use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}
