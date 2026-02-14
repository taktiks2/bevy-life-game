//! コンウェイのライフゲーム - アプリケーションエントリーポイント
//!
//! Bevyエンジンを使用したライフゲームの実装。
//! タイトル画面・ゲーム画面・メニュー画面の3画面構成。

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};

use common::{
    consts::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
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
                        title: "Life Game".to_string(),
                        resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                        resizable: true,
                        resize_constraints: WindowResizeConstraints {
                            min_width: MIN_WINDOW_WIDTH,
                            min_height: MIN_WINDOW_HEIGHT,
                            ..default()
                        },
                        fit_canvas_to_parent: true, // NOTE: WASM環境でcanvasを親要素に追従させリサイズ可能にする
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
