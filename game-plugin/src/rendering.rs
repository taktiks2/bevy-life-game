//! チャンクベースのグリッド描画ユーティリティ

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use common::consts::{
    CELL_ALIVE_RGB, CELL_DEAD_RGB, CELL_PIXELS, CELL_WORLD_SIZE, CHUNK_SIZE, CHUNK_TEX_SIZE,
    CHUNK_WORLD_SIZE, GRID_LINE_RGB, GRID_LINE_SCREEN_WIDTH,
};

use crate::components::chunk::Chunk;
use crate::components::screen::{CellHighlight, OnGameScreen};
use crate::grid_material::{GridMaterial, GridUniforms};
use crate::layer::Layer;
use crate::resources::world::{ChunkKey, World};

/// チャンクのワールド空間位置を計算する（メッシュ中心座標）
pub fn chunk_world_pos(chunk_key: ChunkKey) -> Vec3 {
    let (cx, cy) = chunk_key;
    Vec3::new(
        cx as f32 * CHUNK_WORLD_SIZE + CHUNK_WORLD_SIZE / 2.0,
        -(cy as f32 * CHUNK_WORLD_SIZE + CHUNK_WORLD_SIZE / 2.0),
        0.0,
    )
}

/// チャンクのメッシュエンティティを生成する
///
/// セルデータはテクスチャに、グリッド線はカスタムシェーダーで描画する。
pub fn spawn_chunk_mesh(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<GridMaterial>,
    world: &World,
    chunk_key: ChunkKey,
    camera_scale: f32,
    grid_visible: bool,
) -> Entity {
    let tex_size = CHUNK_TEX_SIZE;
    let mut data = vec![255u8; (tex_size * tex_size * 4) as usize];

    write_chunk_to_image_data(&mut data, world, chunk_key);

    let mut image = Image::new(
        bevy::render::render_resource::Extent3d {
            width: tex_size,
            height: tex_size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());

    let image_handle = images.add(image);

    let material = materials.add(GridMaterial {
        cell_texture: image_handle,
        uniforms: GridUniforms {
            camera_scale,
            grid_visible: if grid_visible { 1.0 } else { 0.0 },
            chunk_cells: CHUNK_SIZE as f32,
            grid_line_width: GRID_LINE_SCREEN_WIDTH,
            grid_color: LinearRgba::new(
                GRID_LINE_RGB.0 as f32 / 255.0,
                GRID_LINE_RGB.1 as f32 / 255.0,
                GRID_LINE_RGB.2 as f32 / 255.0,
                1.0,
            ),
        },
    });

    let mesh = meshes.add(Rectangle::new(CHUNK_WORLD_SIZE, CHUNK_WORLD_SIZE));
    let pos = chunk_world_pos(chunk_key);

    commands
        .spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(pos),
            Layer::World.as_render_layer(),
            OnGameScreen,
            Chunk(chunk_key),
        ))
        .id()
}

/// マウスホバー時のセルハイライトスプライトを生成する
pub fn spawn_cell_highlight(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.85, 0.45, 0.25),
            custom_size: Some(Vec2::new(CELL_WORLD_SIZE, CELL_WORLD_SIZE)),
            ..default()
        },
        Visibility::Hidden,
        Layer::World.as_render_layer(),
        OnGameScreen,
        CellHighlight,
    ));
}

/// チャンクのセル状態をRGBAピクセルデータに書き込む
///
/// チャンク内のセルデータのみをテクスチャに書き込む。
/// グリッド線はシェーダーで描画するため、テクスチャには含まない。
pub fn write_chunk_to_image_data(data: &mut [u8], world: &World, chunk_key: ChunkKey) {
    let (cx, cy) = chunk_key;
    let base_x = cx * CHUNK_SIZE;
    let base_y = cy * CHUNK_SIZE;
    let tex_w = CHUNK_TEX_SIZE as usize;
    let cp = CELL_PIXELS as usize;

    for tex_y in 0..tex_w {
        for tex_x in 0..tex_w {
            let offset = (tex_y * tex_w + tex_x) * 4;

            let cell_x = base_x + (tex_x / cp).min(CHUNK_SIZE as usize - 1) as i32;
            let cell_y = base_y + (tex_y / cp).min(CHUNK_SIZE as usize - 1) as i32;

            let (r, g, b) = if world.is_alive(cell_x, cell_y) {
                CELL_ALIVE_RGB
            } else {
                CELL_DEAD_RGB
            };

            data[offset] = r;
            data[offset + 1] = g;
            data[offset + 2] = b;
            data[offset + 3] = 255;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::world::World;
    use common::consts::{CELL_ALIVE_RGB, CELL_DEAD_RGB, CELL_PIXELS, CHUNK_TEX_SIZE};

    fn pixel_rgb(data: &[u8], tex_width: usize, tx: usize, ty: usize) -> (u8, u8, u8) {
        let offset = (ty * tex_width + tx) * 4;
        (data[offset], data[offset + 1], data[offset + 2])
    }

    #[test]
    fn first_pixel_is_dead_cell() {
        let world = World::new();
        let tex_w = CHUNK_TEX_SIZE as usize;
        let mut data = vec![0u8; tex_w * tex_w * 4];

        write_chunk_to_image_data(&mut data, &world, (0, 0));

        // グリッド線はシェーダー描画のため、テクスチャの(0,0)はセルデータ
        assert_eq!(pixel_rgb(&data, tex_w, 0, 0), CELL_DEAD_RGB);
    }

    #[test]
    fn alive_cell_pixel() {
        let mut world = World::new();
        world.toggle_cell(0, 0);

        let tex_w = CHUNK_TEX_SIZE as usize;
        let mut data = vec![0u8; tex_w * tex_w * 4];

        write_chunk_to_image_data(&mut data, &world, (0, 0));

        assert_eq!(pixel_rgb(&data, tex_w, 0, 0), CELL_ALIVE_RGB);
    }

    #[test]
    fn second_cell_starts_at_cell_pixels_offset() {
        let mut world = World::new();
        world.toggle_cell(1, 0);

        let tex_w = CHUNK_TEX_SIZE as usize;
        let mut data = vec![0u8; tex_w * tex_w * 4];

        write_chunk_to_image_data(&mut data, &world, (0, 0));

        let cp = CELL_PIXELS as usize;
        // セル(1,0)の先頭ピクセル
        assert_eq!(pixel_rgb(&data, tex_w, cp, 0), CELL_ALIVE_RGB);
        // セル(0,0)の範囲内はdead
        assert_eq!(pixel_rgb(&data, tex_w, cp - 1, 0), CELL_DEAD_RGB);
    }
}
