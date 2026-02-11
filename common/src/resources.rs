use bevy::prelude::{AssetServer, AudioSource, Font, FromWorld, Handle, Resource, World};

use crate::consts::{DEFAULT_TICK_INTERVAL, WORLD_HEIGHT, WORLD_WIDTH};

#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    pub font_regular: Handle<Font>,
    pub font_bold: Handle<Font>,
    pub audio_hover: Handle<AudioSource>,
    pub tick_interval: f32,
    pub world_width: u16,
    pub world_height: u16,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            font_bold: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
            font_regular: asset_server.load("fonts/NotoSansJP-Regular.ttf"),
            audio_hover: asset_server.load("audios/appear-online.ogg"),
            tick_interval: DEFAULT_TICK_INTERVAL,
            world_width: WORLD_WIDTH,
            world_height: WORLD_HEIGHT,
        }
    }
}
