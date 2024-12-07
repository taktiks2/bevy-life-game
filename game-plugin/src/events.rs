use bevy::prelude::Event;

#[derive(Event)]
pub struct ProgressGenerationEvent;

#[derive(Event)]
pub struct GenerationResetEvent;

#[derive(Event)]
pub struct WorldClearEvent;
