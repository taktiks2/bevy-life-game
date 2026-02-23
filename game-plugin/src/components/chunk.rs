//! チャンクエンティティのマーカーコンポーネント

use bevy::prelude::Component;

use crate::resources::world::ChunkKey;

/// チャンクエンティティのマーカー（チャンク座標を保持）
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chunk(pub ChunkKey);
