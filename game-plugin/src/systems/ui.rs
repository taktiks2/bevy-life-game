//! UI部品のスポーン関数

use bevy::prelude::*;
use common::{
    consts::{
        ACCENT_GREEN, ACTION_BUTTON_HEIGHT, BG_BUTTON, BORDER_RADIUS, BORDER_SUBTLE,
        FONT_SIZE_LARGE, FONT_SIZE_MEDIUM, FONT_SIZE_SMALL, SLIDER_THUMB_SIZE, SLIDER_TRACK_HEIGHT,
        SLIDER_TRACK_WIDTH, TEXT_MUTED, TEXT_PRIMARY,
    },
    resources::GameAssets,
};

use crate::components::{
    action::GameButtonAction,
    screen::GenerationText,
    slider::{SliderKind, SliderThumb, SliderTrack},
};
use crate::systems::slider::{handle_slider_click, handle_slider_drag};

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
                font: game_assets.font.clone(),
                font_size: FONT_SIZE_MEDIUM,
                ..default()
            },
            TextColor(TEXT_MUTED),
        ))
        .with_child((
            TextSpan::new(generation_count.to_string()),
            TextFont {
                font: game_assets.font.clone(),
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

/// ボタングループのレイアウトノードを返す
pub fn button_group_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(6.),
        ..default()
    }
}

/// スライダーUIを生成する
pub fn spawn_slider(
    parent: &mut ChildSpawnerCommands,
    game_assets: &GameAssets,
    label: &str,
    kind: SliderKind,
) {
    let label = label.to_string();
    let font = game_assets.font.clone();
    let thumb_offset = -(SLIDER_THUMB_SIZE - SLIDER_TRACK_HEIGHT) / 2.0;

    parent
        .spawn(Node {
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.),
            height: Val::Px(ACTION_BUTTON_HEIGHT),
            ..default()
        })
        .with_children(|p| {
            // ラベル
            p.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(TEXT_MUTED),
            ));

            // トラック
            p.spawn((
                Node {
                    width: Val::Px(SLIDER_TRACK_WIDTH),
                    height: Val::Px(SLIDER_TRACK_HEIGHT),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(SLIDER_TRACK_HEIGHT / 2.0)),
                    position_type: PositionType::Relative,
                    ..default()
                },
                BorderColor::all(BORDER_SUBTLE),
                BackgroundColor(BG_BUTTON),
                SliderTrack,
                kind,
            ))
            .observe(handle_slider_drag)
            .observe(handle_slider_click)
            .with_children(|track| {
                // サム
                track.spawn((
                    Node {
                        width: Val::Px(SLIDER_THUMB_SIZE),
                        height: Val::Px(SLIDER_THUMB_SIZE),
                        position_type: PositionType::Absolute,
                        top: Val::Px(thumb_offset),
                        left: Val::Px(0.0),
                        border_radius: BorderRadius::all(Val::Px(SLIDER_THUMB_SIZE / 2.0)),
                        ..default()
                    },
                    BackgroundColor(ACCENT_GREEN),
                    SliderThumb,
                    kind,
                    Pickable::IGNORE,
                ));
            });
        });
}
