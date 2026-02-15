//! UI部品のスポーン関数

use bevy::prelude::*;
use common::{
    consts::{
        ACCENT_GREEN, ACTION_BUTTON_HEIGHT, BG_BUTTON, BORDER_RADIUS, FONT_SIZE_LARGE,
        FONT_SIZE_MEDIUM, FONT_SIZE_SMALL, TEXT_MUTED,
        TEXT_PRIMARY,
    },
    resources::GameAssets,
};

use crate::components::{action::GameButtonAction, screen::GenerationText};

/// 世代カウンター表示テキストを生成する
pub fn spawn_generation_text(
    parent: &mut ChildSpawnerCommands,
    game_assets: &GameAssets,
    generation_count: u64,
) {
    parent
        .spawn((
            Text::new("Gen: ".to_string()),
            TextFont {
                font: game_assets.font_regular.clone(),
                font_size: FONT_SIZE_MEDIUM,
                ..default()
            },
            TextColor(TEXT_MUTED),
        ))
        .with_child((
            TextSpan::new(generation_count.to_string()),
            TextFont {
                font: game_assets.font_regular.clone(),
                font_size: FONT_SIZE_MEDIUM,
                ..default()
            },
            GenerationText,
            TextColor(ACCENT_GREEN),
        ));
}

/// アクションボタン（フルサイズ）を生成する
pub fn spawn_action_button<'a>(
    parent: &'a mut ChildSpawnerCommands<'_>,
    game_assets: &GameAssets,
    label: &str,
    action: GameButtonAction,
) -> EntityCommands<'a> {
    let font = game_assets.font_bold.clone();
    let label = label.to_string();
    let mut entity = parent.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Auto,
            height: Val::Px(ACTION_BUTTON_HEIGHT),
            padding: UiRect::horizontal(Val::Px(16.)),
            border_radius: BorderRadius::px(
                BORDER_RADIUS,
                BORDER_RADIUS,
                BORDER_RADIUS,
                BORDER_RADIUS,
            ),
            ..default()
        },
        action,
        BackgroundColor(BG_BUTTON),
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
            // テキストをピッキング対象外にし、ホバーイベントが子要素から
            // バブリングしてボタンの Over/Out が二重発火するのを防ぐ
            Pickable::IGNORE,
        ));
    });
    entity
}

/// 小型ボタン（ステッパーの増減ボタン）を生成する
pub fn spawn_small_button<'a>(
    parent: &'a mut ChildSpawnerCommands<'_>,
    game_assets: &GameAssets,
    label: &str,
    action: GameButtonAction,
) -> EntityCommands<'a> {
    let font = game_assets.font_bold.clone();
    let label = label.to_string();
    let mut entity = parent.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(36.),
            height: Val::Px(36.),
            border_radius: BorderRadius::px(
                BORDER_RADIUS,
                BORDER_RADIUS,
                BORDER_RADIUS,
                BORDER_RADIUS,
            ),
            ..default()
        },
        action,
        BackgroundColor(BG_BUTTON),
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
            // テキストをピッキング対象外にし、ホバーイベントが子要素から
            // バブリングしてボタンの Over/Out が二重発火するのを防ぐ
            Pickable::IGNORE,
        ));
    });
    entity
}

/// ステッパーのラベル（Speed / Zoom）を生成する
pub fn spawn_stepper_label(
    parent: &mut ChildSpawnerCommands,
    game_assets: &GameAssets,
    label: &str,
) {
    parent.spawn((
        Text::new(label),
        TextFont {
            font: game_assets.font_regular.clone(),
            font_size: FONT_SIZE_SMALL,
            ..default()
        },
        TextColor(TEXT_MUTED),
    ));
}

/// ボタングループのレイアウトノードを返す
pub fn button_group_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(6.),
        ..default()
    }
}

/// ステッパー行のレイアウトノードを返す
pub fn stepper_row_node() -> Node {
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(8.),
        width: Val::Auto,
        height: Val::Px(ACTION_BUTTON_HEIGHT),
        ..default()
    }
}
