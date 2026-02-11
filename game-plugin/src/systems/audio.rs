use bevy::prelude::*;

use crate::events::PlayAudioEvent;

pub fn play_audios(
    asset_server: Res<AssetServer>,
    mut events: MessageReader<PlayAudioEvent>,
    mut commands: Commands,
) {
    for _ in events.read() {
        commands.spawn((
            AudioPlayer::new(asset_server.load("audios/appear-online.ogg")),
            PlaybackSettings::DESPAWN,
        ));
    }
}
