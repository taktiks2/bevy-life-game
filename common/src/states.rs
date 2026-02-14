//! アプリケーション全体の画面遷移ステート

use bevy::prelude::States;

/// ゲームの画面状態
///
/// タイトル画面 → ゲーム画面 ↔ メニュー画面 の遷移を管理する。
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug, States)]
pub enum GameState {
    /// タイトル画面
    #[default]
    Title,
    /// ゲーム画面（ライフゲーム本体）
    Game,
    /// メニュー画面（設定）
    Menu,
}
