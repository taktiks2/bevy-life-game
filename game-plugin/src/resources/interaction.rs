//! インタラクション関連のリソース

use bevy::prelude::*;

/// 現在マウスがホバーしているセルの座標を保持するリソース
#[derive(Resource, Default, PartialEq)]
pub struct HoveredCell(pub Option<(u16, u16)>);

/// 連続再生防止用のクールダウンタイマー（50ms）
/// TimerMode::Once なので reset() 後に再度50ms経過するまで is_finished() は false を返す
#[derive(Resource)]
pub struct AudioCooldown(pub Timer);

/// グリッドラインの表示/非表示状態を管理するリソース
#[derive(Resource, Default)]
pub struct GridVisible(pub bool);

impl Default for AudioCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Once))
    }
}
