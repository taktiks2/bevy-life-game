//! ゲーム画面のUI要素識別用マーカーコンポーネント

use bevy::prelude::Component;

/// ゲーム画面に属する全エンティティのマーカー（画面遷移時の一括削除に使用）
#[derive(Component)]
pub struct OnGameScreen;

/// 世代カウンター表示テキストのマーカー
#[derive(Component)]
pub struct GenerationText;

/// マウスホバー時のセルハイライト表示のマーカー
#[derive(Component)]
pub struct CellHighlight;
