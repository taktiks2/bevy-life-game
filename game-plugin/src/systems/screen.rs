use bevy::{color::palettes::css::*, prelude::*};
use common::resources::GameAssets;

use crate::components::{
    action::GameButtonAction,
    coordinate::Coordinate,
    screen::{GenerationText, OnGameScreen},
};
use crate::resources::world::World;

pub fn spawn_screen(mut commands: Commands, world: Res<World>, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(GRAY.into()),
            OnGameScreen,
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    width: Val::Percent(20.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.),
                    ..default()
                },
                BackgroundColor(GRAY.into()),
            ))
            .with_children(|p| {
                p.spawn((
                    Text::new("Gen: ".to_string()),
                    TextFont {
                        font: game_assets.font_regular.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(WHITE.into()),
                ))
                .with_child((
                    TextSpan::new(world.generation_count.to_string()),
                    TextFont {
                        font: game_assets.font_regular.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    GenerationText,
                    TextColor(WHITE.into()),
                ));
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.),
                        height: Val::Px(60.),
                        ..default()
                    },
                    Button,
                    GameButtonAction::Start,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Start"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.),
                        height: Val::Px(60.),
                        ..default()
                    },
                    Button,
                    GameButtonAction::Stop,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Stop"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.),
                        height: Val::Px(60.),
                        ..default()
                    },
                    Button,
                    GameButtonAction::Next,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Next"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.),
                        height: Val::Px(60.),
                        ..default()
                    },
                    Button,
                    GameButtonAction::Reset,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Reset"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.),
                        height: Val::Px(60.),
                        ..default()
                    },
                    Button,
                    GameButtonAction::Clear,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Clear"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
            });
            p.spawn((
                Node {
                    display: Display::Grid,
                    width: Val::Percent(80.),
                    height: Val::Percent(100.),
                    grid_template_columns: RepeatedGridTrack::auto(world.width),
                    grid_template_rows: RepeatedGridTrack::auto(world.height),
                    row_gap: Val::Px(1.),
                    column_gap: Val::Px(1.),
                    padding: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                BackgroundColor(GRAY.into()),
            ))
            .with_children(|p| {
                for (y, row) in world.cells.iter().enumerate() {
                    for (x, cell) in row.iter().enumerate() {
                        p.spawn((
                            Button,
                            Coordinate {
                                x: x as u16,
                                y: y as u16,
                            },
                            Node {
                                display: Display::Grid,
                                width: Val::Auto,
                                height: Val::Auto,
                                ..default()
                            },
                            BackgroundColor(cell.get_color()),
                        ));
                    }
                }
            });
        });
}
