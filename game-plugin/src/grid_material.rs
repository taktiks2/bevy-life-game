//! チャンク描画用のカスタム2Dマテリアル
//!
//! セルテクスチャの上にグリッド線をシェーダーでプロシージャル描画する。
//! グリッド線はカメラスケールに応じて幅を調整し、
//! 画面上で常に一定のピクセル幅を保つ。

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

/// グリッド線シェーダーに渡すuniform群
#[derive(ShaderType, Debug, Clone)]
pub struct GridUniforms {
    /// カメラのオーソグラフィックスケール
    pub camera_scale: f32,
    /// グリッド表示フラグ（0.0=非表示, 1.0=表示）
    pub grid_visible: f32,
    /// チャンク1辺のセル数
    pub chunk_cells: f32,
    /// グリッド線のスクリーンピクセル幅
    pub grid_line_width: f32,
    /// グリッド線の色
    pub grid_color: LinearRgba,
}

/// チャンク描画用マテリアル
///
/// セルデータのテクスチャとグリッド線描画用のuniformを持つ。
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    /// チャンクのセルテクスチャ
    #[texture(0)]
    #[sampler(1)]
    pub cell_texture: Handle<Image>,
    /// シェーダーuniform
    #[uniform(2)]
    pub uniforms: GridUniforms,
}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grid_material.wgsl".into()
    }
}
