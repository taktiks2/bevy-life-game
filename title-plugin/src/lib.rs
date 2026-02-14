//! タイトル画面プラグイン
//!
//! アプリ起動時に表示されるタイトル画面を提供する。
//! 「Conway's Game of Life」の見出しとStartボタンを表示し、
//! クリックでゲーム画面に遷移する。

use bevy::{color::palettes::css::*, prelude::*};

use common::{
    resources::GameAssets,
    states::GameState,
    systems::{despawn_entity, setup_camera},
    ui::{spawn_screen_button, spawn_screen_container, spawn_screen_title},
};

/// タイトル画面のBevyプラグイン
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Title),
            (setup_title_screen, setup_title_camera),
        );
        app.add_systems(
            OnExit(GameState::Title),
            (
                despawn_entity::<OnTitleScreen>,
                despawn_entity::<TitleCamera>,
            ),
        );
    }
}

/// タイトル画面に属する全エンティティのマーカー
#[derive(Component)]
struct OnTitleScreen;

/// タイトル画面用カメラのマーカー
#[derive(Component)]
pub struct TitleCamera;

/// タイトル画面のボタンアクション
#[derive(Component)]
enum TitleButtonAction {
    /// ゲーム画面に遷移する
    Start,
}

/// タイトル画面用カメラを生成する
fn setup_title_camera(commands: Commands) {
    setup_camera(commands, TitleCamera);
}

/// タイトル画面のUIを構築する
fn setup_title_screen(mut commands: Commands, game_assets: Res<GameAssets>) {
    spawn_screen_container(&mut commands, OnTitleScreen, WHITE.into()).with_children(|parent| {
        spawn_screen_title(parent, game_assets.font_bold.clone(), "Conway's Game of Life", BLACK.into());
        spawn_screen_button(parent, game_assets.font_bold.clone(), "Start")
            .insert(TitleButtonAction::Start)
            .observe(on_start_button_click);
    });
}

/// Startボタンのクリックハンドラ: ゲーム画面に遷移する
fn on_start_button_click(_click: On<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Game);
}
