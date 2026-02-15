//! インタラクション関連のリソース

use bevy::prelude::*;

/// 現在マウスがホバーしているセルの座標を保持するリソース
#[derive(Resource, Default, PartialEq)]
pub struct HoveredCell(pub Option<(i32, i32)>);

/// 連続再生防止用のクールダウンタイマー（50ms）
/// TimerMode::Once なので reset() 後に再度50ms経過するまで is_finished() は false を返す
#[derive(Resource)]
pub struct AudioCooldown(pub Timer);

/// グリッドラインの表示/非表示状態を管理するリソース
#[derive(Resource, Default)]
pub struct GridVisible(pub bool);

/// マウスドラッグ状態を管理するリソース
///
/// 左クリック＋ドラッグでカメラパンを行うために、
/// クリック開始位置と前フレーム位置を追跡する。
#[derive(Resource, Default)]
pub struct DragState {
    /// ドラッグ開始時のカーソル位置（スクリーン座標）
    pub start_pos: Option<Vec2>,
    /// 前フレームのカーソル位置（スクリーン座標）
    pub last_pos: Option<Vec2>,
    /// ドラッグ判定済みか（閾値を超えたらtrue）
    pub is_dragging: bool,
}

impl Default for AudioCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Once))
    }
}
