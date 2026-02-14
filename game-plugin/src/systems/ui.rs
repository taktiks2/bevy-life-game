//! UI部品のスポーン関数

use bevy::{color::palettes::css::*, prelude::*};
use common::{
    consts::{
        ACTION_BUTTON_HEIGHT, BORDER_RADIUS, FONT_SIZE_LARGE, FONT_SIZE_MEDIUM, FONT_SIZE_SMALL,
    },
    resources::GameAssets,
};

use crate::components::{
    action::GameButtonAction,
    screen::GenerationText,
};

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
            TextColor(WHITE.into()),
        ))
        .with_child((
            TextSpan::new(generation_count.to_string()),
            TextFont {
                font: game_assets.font_regular.clone(),
                font_size: FONT_SIZE_MEDIUM,
                ..default()
            },
            GenerationText,
            TextColor(WHITE.into()),
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
            width: Val::Percent(80.),
            height: Val::Px(ACTION_BUTTON_HEIGHT),
            border_radius: BorderRadius::px(BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS),
            ..default()
        },
        action,
        BackgroundColor(BLACK.into()),
    ));
    entity.with_children(|p| {
        p.spawn((
            Text::new(label),
            TextFont {
                font,
                font_size: FONT_SIZE_LARGE,
                ..default()
            },
            TextColor(WHITE.into()),
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
            width: Val::Percent(25.),
            height: Val::Percent(100.),
            border_radius: BorderRadius::px(BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS, BORDER_RADIUS),
            ..default()
        },
        action,
        BackgroundColor(BLACK.into()),
    ));
    entity.with_children(|p| {
        p.spawn((
            Text::new(label),
            TextFont {
                font,
                font_size: FONT_SIZE_LARGE,
                ..default()
            },
            TextColor(WHITE.into()),
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
        TextColor(WHITE.into()),
    ));
}

/// ステッパー行のレイアウトノードを返す
pub fn stepper_row_node() -> Node {
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(80.),
        height: Val::Px(ACTION_BUTTON_HEIGHT),
        ..default()
    }
}
