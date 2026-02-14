//! 複数プラグインで共有されるUI構築ユーティリティ

use bevy::prelude::*;

use crate::consts::{
    BG_BUTTON, BG_BUTTON_HOVER, BORDER_RADIUS, BORDER_SUBTLE, FONT_SIZE_LARGE, FONT_SIZE_TITLE,
    TEXT_PRIMARY, TITLE_BUTTON_HEIGHT, TITLE_BUTTON_WIDTH, TITLE_PADDING,
};

/// 全画面をカバーするルートコンテナを生成する
///
/// Column / Center / SpaceBetween / TITLE_PADDING のレイアウト。
/// `marker` は画面固有のマーカーコンポーネント（例: `OnTitleScreen`）。
pub fn spawn_screen_container<'a, M: Component>(
    commands: &'a mut Commands,
    marker: M,
    bg_color: Color,
) -> EntityCommands<'a> {
    commands.spawn((
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
        BackgroundColor(bg_color),
        marker,
    ))
}

/// タイトル / メニュー画面用のボタンを生成する
///
/// `TITLE_BUTTON_WIDTH x TITLE_BUTTON_HEIGHT`、ダーク背景、角丸のボタン。
/// `label` のテキストを中央配置する。
pub fn spawn_screen_button<'a>(
    parent: &'a mut ChildSpawnerCommands<'_>,
    font: Handle<Font>,
    label: &str,
) -> EntityCommands<'a> {
    let label = label.to_string();
    let mut entity = parent.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(TITLE_BUTTON_WIDTH),
            height: Val::Px(TITLE_BUTTON_HEIGHT),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::px(
                BORDER_RADIUS,
                BORDER_RADIUS,
                BORDER_RADIUS,
                BORDER_RADIUS,
            ),
            ..default()
        },
        Button,
        BackgroundColor(BG_BUTTON),
        BorderColor::all(BORDER_SUBTLE),
    ));
    entity.with_children(|p| {
        p.spawn((
            Text::new(label),
            TextFont {
                font,
                font_size: FONT_SIZE_LARGE,
                ..default()
            },
            TextColor(TEXT_PRIMARY),
        ));
    });
    entity
}

/// 画面タイトルテキストを生成する
pub fn spawn_screen_title(
    parent: &mut ChildSpawnerCommands<'_>,
    font: Handle<Font>,
    title: &str,
    color: Color,
) {
    parent.spawn((
        Text::new(title.to_string()),
        TextFont {
            font,
            font_size: FONT_SIZE_TITLE,
            ..default()
        },
        TextColor(color),
    ));
}

/// 画面ボタンのホバー時ハンドラ: 背景色を変更する
pub fn handle_screen_button_over(over: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor>) {
    if let Ok(mut background_color) = query.get_mut(over.entity) {
        background_color.0 = BG_BUTTON_HOVER;
    }
}

/// 画面ボタンのホバー終了ハンドラ: 背景色を元に戻す
pub fn handle_screen_button_out(out: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor>) {
    if let Ok(mut background_color) = query.get_mut(out.entity) {
        background_color.0 = BG_BUTTON;
    }
}
