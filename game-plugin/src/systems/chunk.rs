//! チャンクの動的スポーン/デスポーン/更新システム

use bevy::prelude::*;
use common::consts::CHUNK_WORLD_SIZE;
use rustc_hash::FxHashSet;

use crate::WorldCamera;
use crate::components::chunk::Chunk;
use crate::rendering::{spawn_chunk_sprite, write_chunk_to_image_data};
use crate::resources::interaction::GridVisible;
use crate::resources::world::{ChunkKey, World};

/// カメラ位置・スケールからビューポート内のチャンクキー集合を計算する
///
/// ビューポートの1チャンク分のマージンを含めて計算し、
/// スクロール時のチャンク出現遅れを防ぐ。
pub fn calc_visible_chunks(
    camera_pos: Vec2,
    camera_scale: f32,
    viewport_w: f32,
    viewport_h: f32,
) -> FxHashSet<ChunkKey> {
    let half_w = viewport_w * camera_scale / 2.0;
    let half_h = viewport_h * camera_scale / 2.0;

    let min_x = camera_pos.x - half_w;
    let max_x = camera_pos.x + half_w;
    // Y軸反転: カメラY+が上、グリッドY+が下
    let min_y = -(camera_pos.y + half_h);
    let max_y = -(camera_pos.y - half_h);

    let chunk_min_x = (min_x / CHUNK_WORLD_SIZE).floor() as i32 - 1;
    let chunk_max_x = (max_x / CHUNK_WORLD_SIZE).ceil() as i32;
    let chunk_min_y = (min_y / CHUNK_WORLD_SIZE).floor() as i32 - 1;
    let chunk_max_y = (max_y / CHUNK_WORLD_SIZE).ceil() as i32;

    let mut chunks = FxHashSet::default();
    for cy in chunk_min_y..=chunk_max_y {
        for cx in chunk_min_x..=chunk_max_x {
            chunks.insert((cx, cy));
        }
    }
    chunks
}

/// チャンクの動的管理を行うシステム
///
/// カメラのビューポートに基づいて:
/// - 新規に見えるチャンクをスポーン
/// - 範囲外のチャンクをデスポーン
/// - 変更のあったチャンクのテクスチャを再描画
pub fn manage_chunks(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut world: ResMut<World>,
    grid_visible: Res<GridVisible>,
    camera_query: Query<(&Transform, &Projection), With<WorldCamera>>,
    windows: Query<&Window>,
    existing_chunks: Query<(Entity, &Chunk, &Sprite)>,
) {
    let Ok((camera_transform, projection)) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };

    let camera_scale = match projection {
        Projection::Orthographic(ortho) => ortho.scale,
        _ => 1.0,
    };

    let viewport_w = window.width();
    let viewport_h = window.height();
    let camera_pos = camera_transform.translation.truncate();

    let visible_chunks = calc_visible_chunks(camera_pos, camera_scale, viewport_w, viewport_h);

    // 既存チャンクのマップを構築
    let mut existing_map: FxHashSet<ChunkKey> = FxHashSet::default();
    for (_, chunk, _) in &existing_chunks {
        existing_map.insert(chunk.0);
    }

    // ビューポート外のチャンクをデスポーン
    for (entity, chunk, sprite) in &existing_chunks {
        if !visible_chunks.contains(&chunk.0) {
            // Imageアセットも削除してメモリリークを防止
            images.remove(&sprite.image);
            commands.entity(entity).despawn();
        }
    }

    // 新規チャンクをスポーン
    for &chunk_key in &visible_chunks {
        if !existing_map.contains(&chunk_key) {
            spawn_chunk_sprite(
                &mut commands,
                &mut images,
                &world,
                chunk_key,
                grid_visible.0,
            );
        }
    }

    // GridVisible変更時: 全チャンクのテクスチャを再描画
    let force_redraw = grid_visible.is_changed();

    // dirtyチャンクまたはGridVisible変更時のテクスチャ更新
    if world.is_changed() || force_redraw {
        for (_, chunk, sprite) in &existing_chunks {
            if !visible_chunks.contains(&chunk.0) {
                continue;
            }
            let needs_update = force_redraw || world.dirty_chunks().contains(&chunk.0);
            if needs_update {
                if let Some(image) = images.get_mut(&sprite.image) {
                    if let Some(ref mut data) = image.data {
                        write_chunk_to_image_data(data, &world, chunk.0, grid_visible.0);
                    }
                }
            }
        }
    }

    // dirtyチャンクをクリア
    world.clear_dirty_chunks();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_visible_chunks_at_origin() {
        let chunks = calc_visible_chunks(Vec2::ZERO, 1.0, 100.0, 100.0);
        // 原点を中心に100x100ワールドユニット = ~3x3チャンク + マージン
        assert!(chunks.contains(&(0, 0)));
        assert!(chunks.contains(&(-1, -1)));
        assert!(chunks.contains(&(1, 1)));
    }

    #[test]
    fn calc_visible_chunks_offset() {
        // カメラが (100, -100) にある場合（グリッド座標(100, 100)付近）
        let chunks = calc_visible_chunks(Vec2::new(100.0, -100.0), 1.0, 64.0, 64.0);
        // チャンク (3, 3) = セル(96..127, 96..127) 付近
        assert!(chunks.contains(&(3, 3)));
    }

    #[test]
    fn calc_visible_chunks_zoomed_out() {
        // ズームアウト(scale=2.0)でより多くのチャンクが見える
        let chunks_zoomed = calc_visible_chunks(Vec2::ZERO, 2.0, 100.0, 100.0);
        let chunks_normal = calc_visible_chunks(Vec2::ZERO, 1.0, 100.0, 100.0);
        assert!(chunks_zoomed.len() > chunks_normal.len());
    }
}
