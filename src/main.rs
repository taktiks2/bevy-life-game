use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use common::{
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    resources::GameAssets,
    states::GameState,
    systems::setup_camera,
};

use game_plugin::GamePlugin;
use menu_plugin::MenuPlugin;
use title_plugin::TitlePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Conway's Game of Life".to_string(),
                resolution: [WINDOW_WIDTH, WINDOW_HEIGHT].into(), // NOTE: Windowサイズの指定
                resizable: false, // NOTE: Windowサイズの変更を不可にする
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new()) // NOTE: インスペクタープラグイン
        .add_plugins(TitlePlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        .init_resource::<GameAssets>()
        .init_state::<GameState>()
        .run();
}
