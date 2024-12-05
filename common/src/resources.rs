use bevy::prelude::{AssetServer, Font, FromWorld, Handle, Resource, World};

use crate::consts::{WORLD_HEIGHT, WORLD_WIDTH};

#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    pub font_regular: Handle<Font>,
    pub font_bold: Handle<Font>,
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
            tick_interval: 0.2,
            world_width: WORLD_WIDTH,
            world_height: WORLD_HEIGHT,
        }
    }
}
