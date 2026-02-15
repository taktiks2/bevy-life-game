//! チャンクベースのグリッド描画ユーティリティ

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use common::consts::{
    CELL_ALIVE_RGB, CELL_DEAD_RGB, CELL_PIXELS, CELL_WORLD_SIZE, CHUNK_SIZE, CHUNK_TEX_SIZE,
    CHUNK_WORLD_SIZE, GRID_LINE_PIXELS, GRID_LINE_RGB,
};

use crate::components::chunk::Chunk;
use crate::components::screen::{CellHighlight, OnGameScreen};
use crate::layer::Layer;
use crate::resources::world::{ChunkKey, World};

/// チャンクのワールド空間位置を計算する（Sprite中心座標）
pub fn chunk_world_pos(chunk_key: ChunkKey) -> Vec3 {
    let (cx, cy) = chunk_key;
    Vec3::new(
        cx as f32 * CHUNK_WORLD_SIZE + CHUNK_WORLD_SIZE / 2.0,
        -(cy as f32 * CHUNK_WORLD_SIZE + CHUNK_WORLD_SIZE / 2.0),
        0.0,
    )
}

/// チャンクのスプライトエンティティを生成する
pub fn spawn_chunk_sprite(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    world: &World,
    chunk_key: ChunkKey,
    grid_visible: bool,
) -> Entity {
    let tex_size = CHUNK_TEX_SIZE;
    let mut data = vec![255u8; (tex_size * tex_size * 4) as usize];

    write_chunk_to_image_data(&mut data, world, chunk_key, grid_visible);

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

    let handle = images.add(image);
    let pos = chunk_world_pos(chunk_key);

    commands
        .spawn((
            Sprite {
                image: handle,
                custom_size: Some(Vec2::new(CHUNK_WORLD_SIZE, CHUNK_WORLD_SIZE)),
                ..default()
            },
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
/// チャンク内の32×32セルのピクセルデータを生成する。
/// グリッド線はテクスチャ内の1pxマージンとして描画。
pub fn write_chunk_to_image_data(
    data: &mut [u8],
    world: &World,
    chunk_key: ChunkKey,
    grid_visible: bool,
) {
    let (cx, cy) = chunk_key;
    let base_x = cx * CHUNK_SIZE;
    let base_y = cy * CHUNK_SIZE;
    let tex_w = CHUNK_TEX_SIZE as usize;
    let gl = GRID_LINE_PIXELS as usize;
    let cp = CELL_PIXELS as usize;
    let stride = cp + gl;

    for tex_y in 0..tex_w {
        for tex_x in 0..tex_w {
            let offset = (tex_y * tex_w + tex_x) * 4;

            // グリッド線かセル領域かを判定
            let is_grid_x = tex_x % stride < gl;
            let is_grid_y = tex_y % stride < gl;

            let (r, g, b) = if grid_visible && (is_grid_x || is_grid_y) {
                GRID_LINE_RGB
            } else {
                let cell_local_x = (tex_x / stride) as i32;
                let cell_local_y = (tex_y / stride) as i32;
                let cell_x = base_x + cell_local_x.min(CHUNK_SIZE - 1);
                let cell_y = base_y + cell_local_y.min(CHUNK_SIZE - 1);

                if world.is_alive(cell_x, cell_y) {
                    CELL_ALIVE_RGB
                } else {
                    CELL_DEAD_RGB
                }
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
    use common::consts::{
        CELL_ALIVE_RGB, CELL_DEAD_RGB, CELL_PIXELS, CHUNK_TEX_SIZE, GRID_LINE_PIXELS, GRID_LINE_RGB,
    };

    fn pixel_rgb(data: &[u8], tex_width: usize, tx: usize, ty: usize) -> (u8, u8, u8) {
        let offset = (ty * tex_width + tx) * 4;
        (data[offset], data[offset + 1], data[offset + 2])
    }

    #[test]
    fn grid_visible_top_left_is_grid_line() {
        let world = World::new();
        let tex_w = CHUNK_TEX_SIZE as usize;
        let mut data = vec![0u8; tex_w * tex_w * 4];

        write_chunk_to_image_data(&mut data, &world, (0, 0), true);

        assert_eq!(pixel_rgb(&data, tex_w, 0, 0), GRID_LINE_RGB);
    }

    #[test]
    fn grid_visible_first_cell_pixel() {
        let world = World::new();
        let tex_w = CHUNK_TEX_SIZE as usize;
        let mut data = vec![0u8; tex_w * tex_w * 4];

        write_chunk_to_image_data(&mut data, &world, (0, 0), true);

        let gl = GRID_LINE_PIXELS as usize;
        assert_eq!(pixel_rgb(&data, tex_w, gl, gl), CELL_DEAD_RGB);
    }

    #[test]
    fn grid_visible_alive_cell() {
        let mut world = World::new();
        world.toggle_cell(0, 0);

        let tex_w = CHUNK_TEX_SIZE as usize;
        let mut data = vec![0u8; tex_w * tex_w * 4];

        write_chunk_to_image_data(&mut data, &world, (0, 0), true);

        let gl = GRID_LINE_PIXELS as usize;
        assert_eq!(pixel_rgb(&data, tex_w, gl, gl), CELL_ALIVE_RGB);
    }

    #[test]
    fn grid_hidden_grid_line_pixels_are_dead_color() {
        let world = World::new();
        let tex_w = CHUNK_TEX_SIZE as usize;
        let mut data = vec![0u8; tex_w * tex_w * 4];

        write_chunk_to_image_data(&mut data, &world, (0, 0), false);

        assert_eq!(pixel_rgb(&data, tex_w, 0, 0), CELL_DEAD_RGB);
    }

    #[test]
    fn grid_line_between_cells() {
        let world = World::new();
        let tex_w = CHUNK_TEX_SIZE as usize;
        let mut data = vec![0u8; tex_w * tex_w * 4];

        write_chunk_to_image_data(&mut data, &world, (0, 0), true);

        let gl = GRID_LINE_PIXELS as usize;
        let cp = CELL_PIXELS as usize;
        let grid_x = gl + cp; // セル(0,0)の右端の次がグリッド線
        assert_eq!(pixel_rgb(&data, tex_w, grid_x, gl), GRID_LINE_RGB);
    }
}
