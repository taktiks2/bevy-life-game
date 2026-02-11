use bevy::prelude::*;
use common::resources::GameAssets;

use crate::events::PlayAudioEvent;

#[derive(Resource)]
pub struct AudioCooldown(pub Timer);

impl Default for AudioCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Once))
    }
}

pub fn play_audios(
    game_assets: Res<GameAssets>,
    mut events: MessageReader<PlayAudioEvent>,
    mut commands: Commands,
    mut cooldown: ResMut<AudioCooldown>,
    time: Res<Time>,
) {
    cooldown.0.tick(time.delta());
    for _ in events.read() {
        if cooldown.0.is_finished() {
            commands.spawn((
                AudioPlayer::new(game_assets.audio_hover.clone()),
                PlaybackSettings::DESPAWN,
            ));
            cooldown.0.reset();
        }
    }
}
