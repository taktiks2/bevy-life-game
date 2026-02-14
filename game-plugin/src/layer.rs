//! レンダリングレイヤーの定義
//!
//! サイドメニューとワールドを別カメラで描画するためのレイヤー分離。

use bevy::camera::visibility::RenderLayers;

/// 描画レイヤーの種類
///
/// 各エンティティとカメラに付与し、対応するカメラのみで描画されるようにする。
#[derive(Clone, Copy)]
pub enum Layer {
    /// サイドメニュー（操作パネル）用レイヤー
    SideMenu,
    /// ワールド（セルグリッド）用レイヤー
    World,
}

impl Layer {
    /// Bevy の `RenderLayers` に変換する
    pub fn as_render_layer(self) -> RenderLayers {
        RenderLayers::layer(self as usize)
    }
}
