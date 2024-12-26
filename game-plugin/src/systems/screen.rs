use bevy::{color::palettes::css::*, prelude::*};
use common::{
    consts::{
        CELL_HEIGHT, CELL_WIDTH, INTERVAL_HEIGHT, INTERVAL_WIDTH, OFFSET_HEIGHT, OFFSET_WIDTH,
        WORLD_HEIGHT, WORLD_WIDTH,
    },
    resources::GameAssets,
};

use crate::components::{
    action::GameButtonAction,
    coordinate::Coordinate,
    screen::{GenerationText, OnGameScreen},
};
use crate::layer::Layer;
use crate::resources::world::World;
use crate::systems::action::*;

pub fn spawn_screen(
    mut commands: Commands,
    world: Res<World>,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // NOTE: Side Menu
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
            Layer::SideMenu.as_render_layer(),
            OnGameScreen,
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
                    width: Val::Percent(80.),
                    height: Val::Px(60.),
                    ..default()
                },
                GameButtonAction::Start,
                BorderRadius::px(5., 5., 5., 5.),
                BackgroundColor(BLACK.into()),
            ))
            .observe(handle_start)
            .observe(handle_over)
            .observe(handle_out)
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
                    width: Val::Percent(80.),
                    height: Val::Px(60.),
                    ..default()
                },
                GameButtonAction::Stop,
                BorderRadius::px(5., 5., 5., 5.),
                BackgroundColor(BLACK.into()),
            ))
            .observe(handle_stop)
            .observe(handle_over)
            .observe(handle_out)
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
                    width: Val::Percent(80.),
                    height: Val::Px(60.),
                    ..default()
                },
                GameButtonAction::Next,
                BorderRadius::px(5., 5., 5., 5.),
                BackgroundColor(BLACK.into()),
            ))
            .observe(handle_next)
            .observe(handle_over)
            .observe(handle_out)
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
                    width: Val::Percent(80.),
                    height: Val::Px(60.),
                    ..default()
                },
                GameButtonAction::Reset,
                BorderRadius::px(5., 5., 5., 5.),
                BackgroundColor(BLACK.into()),
            ))
            .observe(handle_reset)
            .observe(handle_over)
            .observe(handle_out)
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
                    width: Val::Percent(80.),
                    height: Val::Px(60.),
                    ..default()
                },
                GameButtonAction::Clear,
                BorderRadius::px(5., 5., 5., 5.),
                BackgroundColor(BLACK.into()),
            ))
            .observe(handle_clear)
            .observe(handle_over)
            .observe(handle_out)
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
            p.spawn(Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(80.),
                height: Val::Px(60.),
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(25.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    GameButtonAction::SpeedDown,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .observe(handle_speed_down)
                .observe(handle_over)
                .observe(handle_out)
                .with_children(|p| {
                    p.spawn((
                        Text::new("<"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
                p.spawn((
                    Text::new("Speed"),
                    TextFont {
                        font: game_assets.font_regular.clone(),
                        font_size: 20.,
                        ..default()
                    },
                    TextColor(WHITE.into()),
                ));
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(25.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    GameButtonAction::SpeedUp,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .observe(handle_speed_up)
                .observe(handle_over)
                .observe(handle_out)
                .with_children(|p| {
                    p.spawn((
                        Text::new(">"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
            });
            p.spawn(Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(80.),
                height: Val::Px(60.),
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(25.),
                        height: Val::Percent(80.),
                        ..default()
                    },
                    GameButtonAction::ZoomDown,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .observe(handle_zoom_down)
                .observe(handle_over)
                .observe(handle_out)
                .with_children(|p| {
                    p.spawn((
                        Text::new("<"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
                p.spawn((
                    Text::new("Zoom"),
                    TextFont {
                        font: game_assets.font_regular.clone(),
                        font_size: 20.,
                        ..default()
                    },
                    TextColor(WHITE.into()),
                ));
                p.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(25.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    GameButtonAction::ZoomUp,
                    BorderRadius::px(5., 5., 5., 5.),
                    BackgroundColor(BLACK.into()),
                ))
                .observe(handle_zoom_up)
                .observe(handle_over)
                .observe(handle_out)
                .with_children(|p| {
                    p.spawn((
                        Text::new(">"),
                        TextFont {
                            font: game_assets.font_bold.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(WHITE.into()),
                    ));
                });
            });
        });

    // NOTE: World
    for (y, row) in world.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(CELL_WIDTH, CELL_HEIGHT))),
                    MeshMaterial2d(materials.add(cell.get_color())),
                    Layer::World.as_render_layer(),
                    OnGameScreen,
                    Transform::from_xyz(
                        (x as u16 % WORLD_WIDTH) as f32 * INTERVAL_WIDTH - OFFSET_WIDTH,
                        (y as u16 % WORLD_HEIGHT) as f32 * INTERVAL_HEIGHT - OFFSET_HEIGHT,
                        0.,
                    ),
                    Coordinate {
                        x: x as u16,
                        y: y as u16,
                    },
                ))
                .observe(switch_cell_state)
                .observe(handle_over)
                .observe(handle_out);
        }
    }
}
