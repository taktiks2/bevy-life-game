//! シミュレーションの実行状態

use bevy::prelude::States;

/// シミュレーションの実行/一時停止を表すステート
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug, States)]
pub enum SimulationState {
    /// 一時停止中（手動操作のみ受付）
    #[default]
    Paused,
    /// 自動シミュレーション実行中
    Simulating,
}
