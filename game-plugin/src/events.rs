use bevy::prelude::Message;

#[derive(Message)]
pub struct ProgressGenerationEvent;

#[derive(Message)]
pub struct GenerationResetEvent;

#[derive(Message)]
pub struct WorldClearEvent;

#[derive(Message)]
pub struct PlayAudioEvent;
