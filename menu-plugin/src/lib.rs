//! メニュー画面プラグイン
//!
//! ゲーム中にEscapeキーで遷移するメニュー画面を提供する。
//! Back（タイトルに戻る）とQuit（アプリ終了）のボタンを表示する。

use bevy::prelude::*;

use common::{
    consts::{BG_DARK, TEXT_PRIMARY, TITLE_BUTTON_WIDTH},
    resources::{AudioMuted, GameAssets},
    states::GameState,
    systems::{despawn_entity, setup_camera},
    ui::{
        handle_screen_button_out, handle_screen_button_over, spawn_screen_button,
        spawn_screen_container, spawn_screen_title,
    },
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

/// ミュート状態に応じたボタンラベルを返す
fn mute_button_label(muted: bool) -> String {
    if muted {
        "Sound: OFF".to_string()
    } else {
        "Sound: ON".to_string()
    }
}

/// メニュー画面のUIを構築する
fn setup_menu_screen(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    audio_muted: Res<AudioMuted>,
) {
    let mute_label = mute_button_label(audio_muted.0);
    spawn_screen_container(&mut commands, OnMenuScreen, BG_DARK).with_children(|parent| {
        spawn_screen_title(
            parent,
            game_assets.font_bold.clone(),
            "Settings",
            TEXT_PRIMARY,
        );
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
                spawn_mute_button(p, game_assets.font_bold.clone(), &mute_label);
                spawn_screen_button(p, game_assets.font_bold.clone(), "Back")
                    .insert(MenuButtonAction::Back)
                    .observe(on_back_button_click)
                    .observe(handle_screen_button_over)
                    .observe(handle_screen_button_out);
                spawn_screen_button(p, game_assets.font_bold.clone(), "Quit")
                    .insert(MenuButtonAction::Quit)
                    .observe(on_quit_button_click)
                    .observe(handle_screen_button_over)
                    .observe(handle_screen_button_out);
            });
    });
}

/// ミュートトグルボタンを生成する
fn spawn_mute_button(
    parent: &mut ChildSpawnerCommands<'_>,
    font: Handle<Font>,
    label: &str,
) {
    spawn_screen_button(parent, font, label)
        .observe(on_mute_button_click)
        .observe(handle_screen_button_over)
        .observe(handle_screen_button_out);
}

/// ミュートボタンのクリックハンドラ: ミュート状態を切り替え、テキストを更新する
fn on_mute_button_click(
    click: On<Pointer<Click>>,
    mut audio_muted: ResMut<AudioMuted>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text>,
) {
    audio_muted.0 = !audio_muted.0;
    let new_label = mute_button_label(audio_muted.0);

    // ボタンの子要素からTextを見つけて更新
    if let Ok(children) = children_query.get(click.entity) {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                **text = new_label.clone();
            }
        }
    }
}

/// Backボタンのクリックハンドラ: ゲーム画面に戻る
fn on_back_button_click(_click: On<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Game);
}

/// Quitボタンのクリックハンドラ: アプリケーションを終了する
fn on_quit_button_click(_click: On<Pointer<Click>>, mut app_exit_events: MessageWriter<AppExit>) {
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
