use bevy::prelude::{AssetServer, Font, FromWorld, Handle, Resource, World};

#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    pub font_regular: Handle<Font>,
    pub font_bold: Handle<Font>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            font_bold: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
            font_regular: asset_server.load("fonts/NotoSansJP-Regular.ttf"),
        }
    }
}
