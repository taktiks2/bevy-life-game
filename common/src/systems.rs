use bevy::prelude::*;

use crate::resources::GameAssets;

pub fn despawn_screen<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_game_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        font_bold: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
        font_regular: asset_server.load("fonts/NotoSansJP-Regular.ttf"),
        test: "its working".to_string(),
    })
}
