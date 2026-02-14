//! カメラ識別用マーカーコンポーネント

use bevy::prelude::Component;

/// サイドメニュー描画用カメラのマーカー
#[derive(Component)]
pub struct SideMenuCamera;

/// ワールド（セルグリッド）描画用カメラのマーカー
#[derive(Component)]
pub struct WorldCamera;
