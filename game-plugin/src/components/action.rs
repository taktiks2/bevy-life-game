//! サイドメニューのボタンアクション定義

use bevy::prelude::Component;

/// サイドメニューの各ボタンに対応するアクション
///
/// ボタンエンティティにコンポーネントとして付与し、
/// Observerでクリックイベントを処理する。
#[derive(Component)]
pub enum GameButtonAction {
    /// シミュレーション開始/停止トグル
    ToggleSimulation,
    /// 1世代進める
    Next,
    /// 初期状態にリセット
    Reset,
    /// 全セルクリア
    Clear,
    /// グリッドライン表示/非表示切り替え
    GridToggle,
}
