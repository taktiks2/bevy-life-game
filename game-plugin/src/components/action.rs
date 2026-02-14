//! サイドメニューのボタンアクション定義

use bevy::prelude::Component;

/// サイドメニューの各ボタンに対応するアクション
///
/// ボタンエンティティにコンポーネントとして付与し、
/// Observerでクリックイベントを処理する。
#[derive(Component)]
pub enum GameButtonAction {
    /// シミュレーション開始
    Start,
    /// シミュレーション停止
    Stop,
    /// 1世代進める
    Next,
    /// 初期状態にリセット
    Reset,
    /// 全セルクリア
    Clear,
    /// シミュレーション速度を上げる（ティック間隔を短縮）
    SpeedUp,
    /// シミュレーション速度を下げる（ティック間隔を延長）
    SpeedDown,
    /// ズームイン（カメラスケール縮小）
    ZoomUp,
    /// ズームアウト（カメラスケール拡大）
    ZoomDown,
}
