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
use crate::systems::action::{handle_out, handle_over, switch_cell_state};

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
                font_size: 30.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ))
        .with_child((
            TextSpan::new(generation_count.to_string()),
            TextFont {
                font: game_assets.font_regular.clone(),
                font_size: 30.0,
                ..default()
            },
            GenerationText,
            TextColor(WHITE.into()),
        ));
}

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
            height: Val::Px(60.),
            border_radius: BorderRadius::px(5., 5., 5., 5.),
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
                font_size: 40.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ));
    });
    entity
}

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
            border_radius: BorderRadius::px(5., 5., 5., 5.),
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
                font_size: 40.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ));
    });
    entity
}

pub fn spawn_stepper_label(
    parent: &mut ChildSpawnerCommands,
    game_assets: &GameAssets,
    label: &str,
) {
    parent.spawn((
        Text::new(label),
        TextFont {
            font: game_assets.font_regular.clone(),
            font_size: 20.,
            ..default()
        },
        TextColor(WHITE.into()),
    ));
}

pub fn stepper_row_node() -> Node {
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(80.),
        height: Val::Px(60.),
        ..default()
    }
}

pub fn spawn_cell_grid(
    commands: &mut Commands,
    world: &World,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
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
