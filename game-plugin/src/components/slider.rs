//! スライダーUIコンポーネント

use bevy::prelude::Component;

/// スライダーの種類
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum SliderKind {
    /// シミュレーション速度
    Speed,
    /// カメラズーム
    Zoom,
}

/// スライダーのトラック（背景バー）マーカー
#[derive(Component)]
pub struct SliderTrack;

/// スライダーのサム（つまみ）マーカー
#[derive(Component)]
pub struct SliderThumb;
