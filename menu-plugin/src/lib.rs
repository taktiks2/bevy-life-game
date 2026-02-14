//! メニュー画面プラグイン
//!
//! ゲーム中にEscapeキーで遷移するメニュー画面を提供する。
//! Back（タイトルに戻る）とQuit（アプリ終了）のボタンを表示する。

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

/// メニュー画面のBevyプラグイン
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            (setup_menu_screen, setup_menu_camera),
        );
        app.add_systems(
            OnExit(GameState::Menu),
            (despawn_entity::<OnMenuScreen>, despawn_entity::<MenuCamera>),
        );
        app.add_systems(
            Update,
            menu_input_keyboard_handling.run_if(in_state(GameState::Menu)),
        );
    }
}

/// メニュー画面に属する全エンティティのマーカー
#[derive(Component)]
struct OnMenuScreen;

/// メニュー画面用カメラのマーカー
#[derive(Component)]
struct MenuCamera;

/// メニュー画面のボタンアクション
#[derive(Component)]
enum MenuButtonAction {
    /// タイトル画面に戻る
    Back,
    /// アプリケーションを終了する
    Quit,
}

/// メニュー画面用カメラを生成する
fn setup_menu_camera(commands: Commands) {
    setup_camera(commands, MenuCamera);
}

/// メニュー画面のUIを構築する
fn setup_menu_screen(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(TITLE_PADDING),
                    bottom: Val::Px(TITLE_PADDING),
                },
                ..default()
            },
            OnMenuScreen,
            BackgroundColor(GRAY.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Settings"),
                TextFont {
                    font: game_assets.font_bold.clone(),
                    font_size: FONT_SIZE_TITLE,
                    ..default()
                },
            ));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Px(TITLE_BUTTON_WIDTH),
                    height: Val::Px(TITLE_BUTTON_WIDTH),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn((
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(TITLE_BUTTON_WIDTH),
                            height: Val::Px(TITLE_BUTTON_HEIGHT),
                            border_radius: BorderRadius::px(BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS),
                            ..default()
                        },
                        MenuButtonAction::Back,
                        BackgroundColor(BLACK.into()),
                        Button,
                    ))
                    .observe(on_back_button_click)
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Back"),
                            TextFont {
                                font: game_assets.font_bold.clone(),
                                font_size: FONT_SIZE_LARGE,
                                ..default()
                            },
                        ));
                    });
                    p.spawn((
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(TITLE_BUTTON_WIDTH),
                            height: Val::Px(TITLE_BUTTON_HEIGHT),
                            border_radius: BorderRadius::px(BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS),
                            ..default()
                        },
                        MenuButtonAction::Quit,
                        BackgroundColor(BLACK.into()),
                        Button,
                    ))
                    .observe(on_quit_button_click)
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Quit"),
                            TextFont {
                                font: game_assets.font_bold.clone(),
                                font_size: FONT_SIZE_LARGE,
                                ..default()
                            },
                        ));
                    });
                });
        });
}

/// Backボタンのクリックハンドラ: タイトル画面に遷移する
fn on_back_button_click(_click: On<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Title);
}

/// Quitボタンのクリックハンドラ: アプリケーションを終了する
fn on_quit_button_click(
    _click: On<Pointer<Click>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    app_exit_events.write(AppExit::Success);
}

/// メニュー画面のキーボード入力ハンドラ: Escapeでゲーム画面に戻る
pub fn menu_input_keyboard_handling(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        state.set(GameState::Game);
    }
}
