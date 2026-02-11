use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::*,
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use common::{
    consts::{MAIN_PHYSICAL_WIDTH, WINDOW_HEIGHT},
    resources::GameAssets,
};

use crate::components::{
    action::GameButtonAction,
    screen::{CellHighlight, GenerationText, GridTexture, OnGameScreen},
};
use crate::layer::Layer;
use crate::resources::world::World;

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

pub fn spawn_grid_sprite(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    world: &World,
) {
    let width = world.width as u32;
    let height = world.height as u32;
    let mut data = vec![255u8; (width * height * 4) as usize];
    write_world_to_image_data(&mut data, world);

    let mut image = Image::new(
        bevy::render::render_resource::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());

    let handle = images.add(image);

    commands.spawn((
        Sprite {
            image: handle,
            custom_size: Some(Vec2::new(
                MAIN_PHYSICAL_WIDTH as f32,
                WINDOW_HEIGHT,
            )),
            ..default()
        },
        Layer::World.as_render_layer(),
        OnGameScreen,
        GridTexture,
    ));
}

pub fn spawn_cell_highlight(commands: &mut Commands) {
    let cell_w = MAIN_PHYSICAL_WIDTH as f32 / 100.0;
    let cell_h = WINDOW_HEIGHT / 100.0;
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.5, 0.3),
            custom_size: Some(Vec2::new(cell_w, cell_h)),
            ..default()
        },
        Visibility::Hidden,
        Layer::World.as_render_layer(),
        OnGameScreen,
        CellHighlight,
    ));
}

pub fn write_world_to_image_data(data: &mut [u8], world: &World) {
    let width = world.width as usize;
    let height = world.height as usize;
    for y in 0..height {
        for x in 0..width {
            let offset = (y * width + x) * 4;
            let (r, g, b) = if world.is_alive(x as u16, y as u16) {
                (0u8, 0u8, 0u8)
            } else {
                (255u8, 255u8, 255u8)
            };
            data[offset] = r;
            data[offset + 1] = g;
            data[offset + 2] = b;
            data[offset + 3] = 255;
        }
    }
}
