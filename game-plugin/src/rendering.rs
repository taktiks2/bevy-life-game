//! グリッド描画ユーティリティ

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use common::consts::{MAIN_PHYSICAL_WIDTH, WINDOW_HEIGHT, WORLD_HEIGHT, WORLD_WIDTH, cell_size};

use crate::components::screen::{CellHighlight, GridTexture, OnGameScreen};
use crate::layer::Layer;
use crate::resources::world::World;

/// セルグリッドのテクスチャスプライトを生成する
///
/// 1セル=1ピクセルのRGBA画像を作成し、`ImageSamplerDescriptor::nearest()` で
/// ピクセルパーフェクトに拡大表示する。
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

/// マウスホバー時のセルハイライトスプライトを生成する
pub fn spawn_cell_highlight(commands: &mut Commands) {
    let (cell_w, cell_h) = cell_size(WORLD_WIDTH, WORLD_HEIGHT);
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

/// Worldのセル状態をRGBAピクセルデータに書き込む
///
/// 生存セル=黒(0,0,0)、死亡セル=白(255,255,255)、アルファは常に255。
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
