//! カメラ識別用マーカーコンポーネント

use bevy::prelude::Component;

/// ボトムパネル描画用カメラのマーカー
#[derive(Component)]
pub struct BottomPanelCamera;

/// ワールド（セルグリッド）描画用カメラのマーカー
#[derive(Component)]
pub struct WorldCamera;
