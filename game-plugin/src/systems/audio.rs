use bevy::prelude::*;
use common::resources::GameAssets;

use crate::events::PlayAudioEvent;

pub fn play_audios(
    game_assets: Res<GameAssets>,
    mut events: MessageReader<PlayAudioEvent>,
    mut commands: Commands,
) {
    for _ in events.read() {
        commands.spawn((
            AudioPlayer::new(game_assets.audio_hover.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
