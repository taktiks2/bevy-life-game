//! タイトル画面プラグイン
//!
//! アプリ起動時に表示されるタイトル画面を提供する。
//! 「Conway's Game of Life」の見出しとStartボタンを表示し、
//! クリックでゲーム画面に遷移する。

use bevy::prelude::*;

use common::{
    consts::{ACCENT_GREEN, BG_DARK},
    resources::GameAssets,
    states::GameState,
    systems::{despawn_entity, setup_camera},
    ui::{
        handle_screen_button_out, handle_screen_button_over, spawn_screen_button,
        spawn_screen_container, spawn_screen_title,
    },
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
    spawn_screen_container(&mut commands, OnTitleScreen, BG_DARK).with_children(|parent| {
        spawn_screen_title(
            parent,
            game_assets.font_bold.clone(),
            "Life Game",
            ACCENT_GREEN,
        );
        spawn_screen_button(parent, game_assets.font_bold.clone(), "Start")
            .insert(TitleButtonAction::Start)
            .observe(on_start_button_click)
            .observe(handle_screen_button_over)
            .observe(handle_screen_button_out);
    });
}

/// Startボタンのクリックハンドラ: ゲーム画面に遷移する
fn on_start_button_click(_click: On<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Game);
}
