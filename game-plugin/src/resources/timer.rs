//! シミュレーション制御用タイマーリソース

use bevy::prelude::*;
use common::consts::SPACE_KEY_HOLD_DURATION;

/// シミュレーションのティック間隔を制御するリピートタイマー
///
/// タイマー完了ごとに `ProgressGenerationEvent` を発火し、世代を進める。
#[derive(Resource)]
pub struct SimulationTimer(pub Timer);

impl SimulationTimer {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}

/// スペースキー長押し判定用のワンショットタイマー
///
/// スペースキー押下から一定時間経過で「長押し」と判定し、
/// 自動シミュレーションを開始する。
#[derive(Resource)]
pub struct SpaceKeyTimer(pub Timer);

impl SpaceKeyTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(
            SPACE_KEY_HOLD_DURATION,
            TimerMode::Once,
        ))
    }
}
