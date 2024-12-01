use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use common::{states::GameState, systems::setup_camera};

use common::consts::WINDOW_SIZE;
use game_plugin::GamePlugin;
use menu_plugin::MenuPlugin;
use title_plugin::TitlePlugin;

fn main() {
    App::new()
        // NOTE: ゲームウィンドウのセットアップ
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Life Game".to_string(),
                resolution: [WINDOW_SIZE, WINDOW_SIZE].into(), // NOTE: Windowサイズの指定
                resizable: false, // NOTE: Windowサイズの変更を不可にする
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new()) // NOTE: インスペクタープラグイン
        .add_plugins(TitlePlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .run();
}
