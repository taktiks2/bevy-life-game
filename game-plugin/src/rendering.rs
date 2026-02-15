//! グリッド描画ユーティリティ

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use common::consts::{
    CELL_ALIVE_RGB, CELL_DEAD_RGB, GRID_DISPLAY_SIZE, WORLD_HEIGHT, WORLD_WIDTH, cell_size,
};

use crate::components::screen::{CellHighlight, GridTexture, OnGameScreen};
use crate::layer::Layer;
use crate::resources::world::World;

/// セルグリッドのテクスチャスプライトを生成する
///
/// 1セル=1ピクセルのRGBA画像を作成し、`ImageSamplerDescriptor::nearest()` で
/// ピクセルパーフェクトに拡大表示する。
pub fn spawn_grid_sprite(commands: &mut Commands, images: &mut Assets<Image>, world: &World) {
    // Worldの幅・高さをu32に変換（Image生成に必要な型）
    let width = world.width as u32;
    let height = world.height as u32;

    // RGBA(4バイト/ピクセル)のバッファを確保し、全ピクセルのアルファを255で初期化
    let mut data = vec![255u8; (width * height * 4) as usize];

    // Worldのセル状態（生存/死亡）に基づいてピクセルのRGB値を書き込む
    write_world_to_image_data(&mut data, world);

    // Bevy用のImage構造体を構築
    let mut image = Image::new(
        // テクスチャの3D範囲を指定（2Dなのでdepth=1）
        bevy::render::render_resource::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        // 2Dテクスチャとして扱う
        bevy::render::render_resource::TextureDimension::D2,
        // 上で構築したRGBAピクセルデータ
        data,
        // sRGB色空間のRGBA8ビットフォーマット
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        // CPU(MAIN_WORLD)とGPU(RENDER_WORLD)の両方からアクセス可能にする
        // ※シミュレーション更新時にCPU側から毎フレーム書き換えるため両方必要
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    // 最近傍補間（nearest neighbor）を指定し、拡大時にピクセルがぼやけないようにする
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());

    // Bevyのアセットマネージャに画像を登録し、ハンドルを取得
    let handle = images.add(image);

    // スプライトエンティティを生成
    commands.spawn((
        Sprite {
            // 上で登録した画像ハンドルをスプライトのテクスチャとして設定
            image: handle,
            // 1セル=1ピクセルの小さな画像を、GRID_DISPLAY_SIZE四方のワールド単位に拡大表示
            custom_size: Some(Vec2::new(GRID_DISPLAY_SIZE, GRID_DISPLAY_SIZE)),
            ..default()
        },
        // ワールドレイヤーに配置（UIと描画順序を分離するため）
        Layer::World.as_render_layer(),
        // ゲーム画面用マーカー（画面遷移時のクリーンアップ対象）
        OnGameScreen,
        // グリッドテクスチャのマーカー（シミュレーション更新時に画像データを書き換える際の検索用）
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
/// 生存セル=ネオングリーン、死亡セル=ほぼ黒、アルファは常に255。
pub fn write_world_to_image_data(data: &mut [u8], world: &World) {
    let width = world.width as usize;
    let height = world.height as usize;

    // 全セルを行(y)→列(x)の順に走査
    for y in 0..height {
        for x in 0..width {
            // RGBAバッファ内でのバイトオフセットを計算（1ピクセル=4バイト）
            let offset = (y * width + x) * 4;

            // セルの生死に応じてRGB色を選択
            // CELL_ALIVE_RGB: 生存セルの色（ネオングリーン）
            // CELL_DEAD_RGB: 死亡セルの色（ほぼ黒）
            let (r, g, b) = if world.is_alive(x as u16, y as u16) {
                CELL_ALIVE_RGB
            } else {
                CELL_DEAD_RGB
            };

            // RGBAの各チャネルをバッファに書き込む
            data[offset] = r; // 赤チャネル
            data[offset + 1] = g; // 緑チャネル
            data[offset + 2] = b; // 青チャネル
            data[offset + 3] = 255; // アルファチャネル（常に不透明）
        }
    }
}
