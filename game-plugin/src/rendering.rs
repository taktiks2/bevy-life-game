//! グリッド描画ユーティリティ

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use common::consts::{
    CELL_ALIVE_RGB, CELL_DEAD_RGB, CELL_PIXELS, GRID_DISPLAY_HEIGHT, GRID_DISPLAY_WIDTH,
    GRID_LINE_PIXELS, GRID_LINE_RGB, WORLD_HEIGHT, WORLD_WIDTH, cell_size, texture_size,
};

use crate::components::screen::{CellHighlight, GridTexture, OnGameScreen};
use crate::layer::Layer;
use crate::resources::world::World;

/// セルグリッドのテクスチャスプライトを生成する
///
/// 各セルをCELL_PIXELS四方で描画し、セル間にGRID_LINE_PIXELSのグリッド線領域を持つ
/// RGBAテクスチャを作成する。`ImageSamplerDescriptor::nearest()` でピクセルパーフェクトに拡大表示。
pub fn spawn_grid_sprite(commands: &mut Commands, images: &mut Assets<Image>, world: &World) {
    let tex_width = texture_size(world.width);
    let tex_height = texture_size(world.height);

    // RGBA(4バイト/ピクセル)のバッファを確保し、全ピクセルのアルファを255で初期化
    let mut data = vec![255u8; (tex_width * tex_height * 4) as usize];

    // Worldのセル状態に基づいてピクセルのRGB値を書き込む（初期状態はグリッド非表示）
    write_world_to_image_data(&mut data, world, false);

    // Bevy用のImage構造体を構築
    let mut image = Image::new(
        bevy::render::render_resource::Extent3d {
            width: tex_width,
            height: tex_height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    // 拡大時: nearest（ピクセルパーフェクト）、縮小時: linear（グリッド線の省略を防止）
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Linear,
        ..ImageSamplerDescriptor::nearest()
    });

    let handle = images.add(image);

    commands.spawn((
        Sprite {
            image: handle,
            custom_size: Some(Vec2::new(GRID_DISPLAY_WIDTH, GRID_DISPLAY_HEIGHT)),
            ..default()
        },
        Layer::World.as_render_layer(),
        OnGameScreen,
        GridTexture,
    ));
}

/// マウスホバー時のセルハイライトスプライトを生成する
pub fn spawn_cell_highlight(commands: &mut Commands) {
    // グリッドの幅・高さから、1セルあたりのワールド単位サイズを計算
    let (cell_w, cell_h) = cell_size(WORLD_WIDTH, WORLD_HEIGHT);

    commands.spawn((
        Sprite {
            // 半透明の緑色（ネオングリーン系、アルファ0.25で透過）
            color: Color::srgba(0.0, 0.85, 0.45, 0.25),
            // 1セル分の大きさに設定
            custom_size: Some(Vec2::new(cell_w, cell_h)),
            ..default()
        },
        // 初期状態では非表示（マウスがセル上にあるときだけVisibleに切り替える）
        Visibility::Hidden,
        // ワールドレイヤーに配置（グリッドスプライトと同じレイヤー）
        Layer::World.as_render_layer(),
        // ゲーム画面用マーカー（画面遷移時のクリーンアップ対象）
        OnGameScreen,
        // セルハイライト用マーカー（ホバーシステムがこのエンティティを検索・更新するために使用）
        CellHighlight,
    ));
}

/// Worldのセル状態をRGBAピクセルデータに書き込む
///
/// テクスチャ上で各セルはCELL_PIXELS四方、セル間にGRID_LINE_PIXELSのグリッド線を配置。
/// `grid_visible`がtrueの場合、グリッド線をGRID_LINE_RGBで描画する。
/// falseの場合、グリッド線領域も隣接セルの色で塗りつぶす。
pub fn write_world_to_image_data(data: &mut [u8], world: &World, grid_visible: bool) {
    let tex_width = texture_size(world.width) as usize;
    let tex_height = texture_size(world.height) as usize;
    let gl = GRID_LINE_PIXELS as usize;
    let cp = CELL_PIXELS as usize;
    let stride = cp + gl;

    for tex_y in 0..tex_height {
        for tex_x in 0..tex_width {
            let offset = (tex_y * tex_width + tex_x) * 4;

            // グリッド線かセル領域かを判定
            let is_grid_x = tex_x < gl || (tex_x - gl) % stride >= cp;
            let is_grid_y = tex_y < gl || (tex_y - gl) % stride >= cp;

            let (r, g, b) = if grid_visible && (is_grid_x || is_grid_y) {
                GRID_LINE_RGB
            } else {
                // セル領域またはグリッド非表示時: 最も近いセルの色を使用
                let cell_x = if tex_x < gl {
                    0
                } else {
                    (((tex_x - gl) / stride) as u16).min(world.width - 1)
                };
                let cell_y = if tex_y < gl {
                    0
                } else {
                    (((tex_y - gl) / stride) as u16).min(world.height - 1)
                };

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
        CELL_ALIVE_RGB, CELL_DEAD_RGB, CELL_PIXELS, GRID_LINE_PIXELS, GRID_LINE_RGB, texture_size,
    };

    /// テクスチャ上のピクセル(tx,ty)のRGB値を取得するヘルパー
    fn pixel_rgb(data: &[u8], tex_width: usize, tx: usize, ty: usize) -> (u8, u8, u8) {
        let offset = (ty * tex_width + tx) * 4;
        (data[offset], data[offset + 1], data[offset + 2])
    }

    #[test]
    fn grid_visible_top_left_is_grid_line() {
        let world = World::new(10, 10);
        let tex_w = texture_size(10) as usize;
        let tex_h = texture_size(10) as usize;
        let mut data = vec![0u8; tex_w * tex_h * 4];

        write_world_to_image_data(&mut data, &world, true);

        // (0,0)はグリッド線
        assert_eq!(pixel_rgb(&data, tex_w, 0, 0), GRID_LINE_RGB);
    }

    #[test]
    fn grid_visible_first_cell_pixel() {
        let world = World::new(10, 10);
        let tex_w = texture_size(10) as usize;
        let tex_h = texture_size(10) as usize;
        let mut data = vec![0u8; tex_w * tex_h * 4];

        write_world_to_image_data(&mut data, &world, true);

        // 最初のセルピクセルは(GRID_LINE_PIXELS, GRID_LINE_PIXELS)の位置
        let gl = GRID_LINE_PIXELS as usize;
        assert_eq!(pixel_rgb(&data, tex_w, gl, gl), CELL_DEAD_RGB);
    }

    #[test]
    fn grid_visible_alive_cell() {
        let mut world = World::new(10, 10);
        world.toggle_cell(0, 0);

        let tex_w = texture_size(10) as usize;
        let tex_h = texture_size(10) as usize;
        let mut data = vec![0u8; tex_w * tex_h * 4];

        write_world_to_image_data(&mut data, &world, true);

        // セル(0,0)の最初のピクセル → 生存色
        let gl = GRID_LINE_PIXELS as usize;
        assert_eq!(pixel_rgb(&data, tex_w, gl, gl), CELL_ALIVE_RGB);
    }

    #[test]
    fn grid_hidden_grid_line_pixels_are_dead_color() {
        let world = World::new(10, 10);
        let tex_w = texture_size(10) as usize;
        let tex_h = texture_size(10) as usize;
        let mut data = vec![0u8; tex_w * tex_h * 4];

        write_world_to_image_data(&mut data, &world, false);

        // グリッド非表示時、(0,0)はCELL_DEAD_RGBになるべき
        assert_eq!(pixel_rgb(&data, tex_w, 0, 0), CELL_DEAD_RGB);
    }

    #[test]
    fn grid_line_between_cells() {
        let world = World::new(10, 10);
        let tex_w = texture_size(10) as usize;
        let tex_h = texture_size(10) as usize;
        let mut data = vec![0u8; tex_w * tex_h * 4];

        write_world_to_image_data(&mut data, &world, true);

        // セル(0,0)とセル(1,0)の間のグリッド線
        // セル(0,0): x=[GL..GL+CP), セル(1,0): x=[GL+CP+GL..GL+CP+GL+CP)
        // 間のグリッド線: x=GL+CP
        let gl = GRID_LINE_PIXELS as usize;
        let cp = CELL_PIXELS as usize;
        let grid_x = gl + cp; // 最初のセルの右端の次
        assert_eq!(pixel_rgb(&data, tex_w, grid_x, gl), GRID_LINE_RGB);
    }
}
