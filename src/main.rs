//! コンウェイのライフゲーム - アプリケーションエントリーポイント
//!
//! Bevyエンジンを使用したライフゲームの実装。
//! タイトル画面・ゲーム画面・メニュー画面の3画面構成。

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};

use common::{
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    resources::GameAssets,
    states::GameState,
};

use game_plugin::GamePlugin;
use menu_plugin::MenuPlugin;
use title_plugin::TitlePlugin;

/// アプリケーションのエントリーポイント
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title:"Life Game": WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32), // NOTE: Windowサイズの指定
                        resizable: false, // NOTE: Windowサイズの変更を不可にする
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never, // NOTE: これを入れないとwasmでassetsがロードされない
                    ..default()
                }),
        )
        .add_plugins(TitlePlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        .init_resource::<GameAssets>()
        .init_state::<GameState>()
        .run();
}
