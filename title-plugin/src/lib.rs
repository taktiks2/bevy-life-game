//! タイトル画面プラグイン
//!
//! アプリ起動時に表示されるタイトル画面を提供する。
//! 「Conway's Game of Life」の見出しとStartボタンを表示し、
//! クリックでゲーム画面に遷移する。

use bevy::{color::palettes::css::*, prelude::*};

use common::{
    consts::{
        BORDER_RADIUS, FONT_SIZE_LARGE, FONT_SIZE_TITLE, TITLE_BUTTON_HEIGHT, TITLE_BUTTON_WIDTH,
        TITLE_PADDING,
    },
    resources::GameAssets,
    states::GameState,
    systems::{despawn_entity, setup_camera},
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
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(TITLE_PADDING),
                    bottom: Val::Px(TITLE_PADDING),
                },
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(WHITE.into()),
            OnTitleScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Conway's Game of Life"),
                TextFont {
                    font: game_assets.font_bold.clone(),
                    font_size: FONT_SIZE_TITLE,
                    ..default()
                },
                TextColor(BLACK.into()),
            ));
            parent
                .spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(TITLE_BUTTON_WIDTH),
                        height: Val::Px(TITLE_BUTTON_HEIGHT),
                        border_radius: BorderRadius::px(BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS),
                        ..default()
                    },
                    Button,
                    TitleButtonAction::Start,
                    BackgroundColor(BLACK.into()),
                ))
                .observe(on_start_button_click)
                .with_children(|p| {
                    p.spawn((
                        Text::new("Start"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: FONT_SIZE_LARGE,
                            ..default()
                        },
                    ));
                });
        });
}

/// Startボタンのクリックハンドラ: ゲーム画面に遷移する
fn on_start_button_click(_click: On<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Game);
}
