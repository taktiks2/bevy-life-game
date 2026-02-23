//! ライフゲームの無限ワールドリソース

use bevy::prelude::Resource;
use rustc_hash::{FxHashMap, FxHashSet};

use common::consts::{CHUNK_SIZE, SQUARE_COORDINATES};

use super::simulation;

/// チャンクの座標キー (chunk_x, chunk_y)
pub type ChunkKey = (i32, i32);

/// ライフゲームの無限ワールドを表すリソース
///
/// 生存セルのみを `FxHashSet` で管理する。座標は `(i32, i32)` で無限に拡張可能。
/// `dirty_chunks` で変更のあったチャンクを追跡し、レンダリングの最適化に使用する。
#[derive(Resource, Clone, Debug)]
pub struct World {
    /// 生存セルの座標集合
    cells: FxHashSet<(i32, i32)>,
    /// ユーザーが配置した初期パターン（リセット時に復元）
    initial_cells: FxHashSet<(i32, i32)>,
    /// 直前の操作で変更があったチャンクの集合
    dirty_chunks: FxHashSet<ChunkKey>,
    /// 現在の世代数
    pub generation_count: u64,
}

impl World {
    /// 空の無限ワールドを生成する
    pub fn new() -> Self {
        Self {
            cells: FxHashSet::default(),
            initial_cells: FxHashSet::default(),
            dirty_chunks: FxHashSet::default(),
            generation_count: 0,
        }
    }

    /// セル座標からチャンクキーを計算する
    pub fn chunk_key(x: i32, y: i32) -> ChunkKey {
        (x.div_euclid(CHUNK_SIZE), y.div_euclid(CHUNK_SIZE))
    }

    /// 指定座標のセルが生きているかを返す
    pub fn is_alive(&self, x: i32, y: i32) -> bool {
        self.cells.contains(&(x, y))
    }

    /// 指定座標のセルの生死をトグルする
    ///
    /// 初期パターンも同時に更新し、世代カウントを0にリセットする。
    pub fn toggle_cell(&mut self, x: i32, y: i32) {
        let chunk = Self::chunk_key(x, y);
        if self.cells.contains(&(x, y)) {
            self.cells.remove(&(x, y));
            self.initial_cells.remove(&(x, y));
        } else {
            self.cells.insert((x, y));
            self.initial_cells.insert((x, y));
        }
        self.dirty_chunks.insert(chunk);
        self.generation_count = 0;
    }

    /// コンウェイのルールに従い世代を1つ進める
    ///
    /// 生存セルとその隣接セルのみを処理する効率的なアルゴリズム。
    pub fn progress_generation(&mut self) {
        self.generation_count += 1;

        // 候補セル = 生存セル + その8近傍のカウントを構築
        let mut candidates: FxHashMap<(i32, i32), u8> = FxHashMap::default();

        for &(x, y) in &self.cells {
            // 自分自身を候補に
            candidates.entry((x, y)).or_insert(0);
            // 8近傍のカウントを+1
            for &(dy, dx) in &SQUARE_COORDINATES {
                let nx = x + dx as i32;
                let ny = y + dy as i32;
                *candidates.entry((nx, ny)).or_insert(0) += 1;
            }
        }

        let old_cells = std::mem::take(&mut self.cells);
        self.dirty_chunks.clear();

        for ((x, y), count) in candidates {
            let was_alive = old_cells.contains(&(x, y));
            let is_alive = simulation::next_cell_state(was_alive, count as usize);
            if is_alive {
                self.cells.insert((x, y));
            }
            if was_alive != is_alive {
                self.dirty_chunks.insert(Self::chunk_key(x, y));
            }
        }
    }

    /// 初期パターンの状態に復元し、世代カウントを0にリセットする
    pub fn reset(&mut self) {
        let old_cells = std::mem::take(&mut self.cells);
        self.cells = self.initial_cells.clone();
        self.dirty_chunks.clear();

        // 変更のあったチャンクを追跡
        for &(x, y) in old_cells.symmetric_difference(&self.cells) {
            self.dirty_chunks.insert(Self::chunk_key(x, y));
        }
        self.generation_count = 0;
    }

    /// 全セルを死んだ状態にし、初期パターンもクリアする
    pub fn clear(&mut self) {
        // 旧生存セルのチャンクをdirtyに
        for &(x, y) in &self.cells {
            self.dirty_chunks.insert(Self::chunk_key(x, y));
        }
        for &(x, y) in &self.initial_cells {
            self.dirty_chunks.insert(Self::chunk_key(x, y));
        }
        self.cells.clear();
        self.initial_cells.clear();
        self.generation_count = 0;
    }

    /// 生存セルの集合を返す
    #[allow(dead_code)]
    pub fn alive_cells(&self) -> &FxHashSet<(i32, i32)> {
        &self.cells
    }

    /// 直前の操作で変更があったチャンクの集合を返す
    pub fn dirty_chunks(&self) -> &FxHashSet<ChunkKey> {
        &self.dirty_chunks
    }

    /// dirtyチャンクの追跡をクリアする
    pub fn clear_dirty_chunks(&mut self) {
        self.dirty_chunks.clear();
    }

    /// 指定したセル群をワールドに配置する
    ///
    /// セルを生存状態にし、初期パターンにも記録する。
    /// 世代カウントを0にリセットし、対応チャンクをdirtyにする。
    pub fn place_pattern(&mut self, cells: &[(i32, i32)]) {
        for &(x, y) in cells {
            self.cells.insert((x, y));
            self.initial_cells.insert((x, y));
            self.dirty_chunks.insert(Self::chunk_key(x, y));
        }
        self.generation_count = 0;
    }
}

#[cfg(test)]
impl World {
    /// 指定座標のセルの生死状態を設定する（テスト専用）
    pub fn set_alive(&mut self, x: i32, y: i32, alive: bool) {
        if alive {
            self.cells.insert((x, y));
        } else {
            self.cells.remove(&(x, y));
        }
    }

    /// 初期パターンにおいて指定座標のセルが生きているかを返す（テスト専用）
    pub fn is_initial_alive(&self, x: i32, y: i32) -> bool {
        self.initial_cells.contains(&(x, y))
    }

    /// 現在のセル状態を初期パターンとして保存する（テスト専用）
    pub fn save_as_initial(&mut self) {
        self.initial_cells = self.cells.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- World::new ---

    #[test]
    fn new_world_is_empty() {
        let world = World::new();
        assert!(world.alive_cells().is_empty());
    }

    #[test]
    fn new_world_has_zero_generation() {
        let world = World::new();
        assert_eq!(world.generation_count, 0);
    }

    // --- toggle_cell ---

    #[test]
    fn toggle_cell_switches_dead_to_alive() {
        let mut world = World::new();
        world.toggle_cell(1, 1);
        assert!(world.is_alive(1, 1));
    }

    #[test]
    fn toggle_cell_switches_alive_to_dead() {
        let mut world = World::new();
        world.set_alive(1, 1, true);
        world.toggle_cell(1, 1);
        assert!(!world.is_alive(1, 1));
    }

    #[test]
    fn toggle_cell_syncs_initial_cells() {
        let mut world = World::new();
        world.toggle_cell(1, 1);
        assert!(world.is_initial_alive(1, 1));
    }

    #[test]
    fn toggle_cell_resets_generation_count() {
        let mut world = World::new();
        world.generation_count = 5;
        world.toggle_cell(1, 1);
        assert_eq!(world.generation_count, 0);
    }

    #[test]
    fn toggle_cell_does_not_affect_other_initial_cells() {
        let mut world = World::new();
        world.set_alive(0, 0, true);
        world.save_as_initial();

        world.toggle_cell(1, 1);

        assert!(world.is_initial_alive(0, 0));
        assert!(world.is_initial_alive(1, 1));
    }

    #[test]
    fn toggle_cell_negative_coordinates() {
        let mut world = World::new();
        world.toggle_cell(-5, -10);
        assert!(world.is_alive(-5, -10));
        assert!(!world.is_alive(0, 0));
    }

    #[test]
    fn toggle_cell_marks_dirty_chunk() {
        let mut world = World::new();
        world.toggle_cell(3, 5);
        assert!(world.dirty_chunks().contains(&World::chunk_key(3, 5)));
    }

    // --- Conway's ルール (progress_generation 経由) ---

    #[test]
    fn alive_cell_with_0_neighbors_dies() {
        let mut world = World::new();
        world.set_alive(1, 1, true);
        world.progress_generation();
        assert!(!world.is_alive(1, 1));
    }

    #[test]
    fn alive_cell_with_1_neighbor_dies() {
        let mut world = World::new();
        world.set_alive(1, 1, true);
        world.set_alive(0, 0, true);
        world.progress_generation();
        assert!(!world.is_alive(1, 1));
    }

    #[test]
    fn alive_cell_with_2_neighbors_survives() {
        let mut world = World::new();
        world.set_alive(1, 1, true);
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.progress_generation();
        assert!(world.is_alive(1, 1));
    }

    #[test]
    fn alive_cell_with_3_neighbors_survives() {
        let mut world = World::new();
        world.set_alive(1, 1, true);
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.set_alive(2, 0, true);
        world.progress_generation();
        assert!(world.is_alive(1, 1));
    }

    #[test]
    fn alive_cell_with_4_neighbors_dies() {
        let mut world = World::new();
        world.set_alive(1, 1, true);
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.set_alive(2, 0, true);
        world.set_alive(0, 1, true);
        world.progress_generation();
        assert!(!world.is_alive(1, 1));
    }

    #[test]
    fn dead_cell_with_3_neighbors_becomes_alive() {
        let mut world = World::new();
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.set_alive(0, 1, true);
        world.progress_generation();
        assert!(world.is_alive(1, 1));
    }

    #[test]
    fn dead_cell_with_2_neighbors_stays_dead() {
        let mut world = World::new();
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.progress_generation();
        assert!(!world.is_alive(1, 1));
    }

    // --- 有名パターン ---

    #[test]
    fn blinker_oscillates() {
        let mut world = World::new();
        world.set_alive(1, 2, true);
        world.set_alive(2, 2, true);
        world.set_alive(3, 2, true);

        world.progress_generation();
        assert!(!world.is_alive(1, 2));
        assert!(world.is_alive(2, 1));
        assert!(world.is_alive(2, 2));
        assert!(world.is_alive(2, 3));
        assert!(!world.is_alive(3, 2));

        world.progress_generation();
        assert!(world.is_alive(1, 2));
        assert!(world.is_alive(2, 2));
        assert!(world.is_alive(3, 2));
        assert!(!world.is_alive(2, 1));
        assert!(!world.is_alive(2, 3));
    }

    #[test]
    fn block_is_stable() {
        let mut world = World::new();
        world.set_alive(1, 1, true);
        world.set_alive(2, 1, true);
        world.set_alive(1, 2, true);
        world.set_alive(2, 2, true);

        world.progress_generation();
        assert!(world.is_alive(1, 1));
        assert!(world.is_alive(2, 1));
        assert!(world.is_alive(1, 2));
        assert!(world.is_alive(2, 2));
        assert_eq!(world.alive_cells().len(), 4);
    }

    // --- generation_count ---

    #[test]
    fn generation_count_increments() {
        let mut world = World::new();
        assert_eq!(world.generation_count, 0);
        world.progress_generation();
        assert_eq!(world.generation_count, 1);
        world.progress_generation();
        assert_eq!(world.generation_count, 2);
    }

    // --- reset ---

    #[test]
    fn reset_restores_initial_cells() {
        let mut world = World::new();
        world.set_alive(0, 0, true);
        world.save_as_initial();

        world.progress_generation();
        world.reset();

        assert!(world.is_alive(0, 0));
        assert_eq!(world.generation_count, 0);
    }

    #[test]
    fn reset_marks_dirty_chunks() {
        let mut world = World::new();
        world.set_alive(0, 0, true);
        world.save_as_initial();
        world.set_alive(50, 50, true);
        world.clear_dirty_chunks();

        world.reset();

        // (50,50)が消えるのでそのチャンクがdirty
        assert!(world.dirty_chunks().contains(&World::chunk_key(50, 50)));
    }

    // --- clear ---

    #[test]
    fn clear_sets_all_cells_dead() {
        let mut world = World::new();
        world.set_alive(0, 0, true);
        world.set_alive(1, 1, true);
        world.generation_count = 5;

        world.clear();

        assert!(world.alive_cells().is_empty());
        assert_eq!(world.generation_count, 0);
    }

    #[test]
    fn clear_marks_dirty_chunks() {
        let mut world = World::new();
        world.set_alive(5, 5, true);
        world.clear_dirty_chunks();

        world.clear();

        assert!(world.dirty_chunks().contains(&World::chunk_key(5, 5)));
    }

    // --- chunk_key ---

    #[test]
    fn chunk_key_positive_coordinates() {
        assert_eq!(World::chunk_key(0, 0), (0, 0));
        assert_eq!(World::chunk_key(63, 63), (0, 0));
        assert_eq!(World::chunk_key(64, 0), (1, 0));
        assert_eq!(World::chunk_key(127, 63), (1, 0));
    }

    #[test]
    fn chunk_key_negative_coordinates() {
        assert_eq!(World::chunk_key(-1, -1), (-1, -1));
        assert_eq!(World::chunk_key(-64, -64), (-1, -1));
        assert_eq!(World::chunk_key(-65, -65), (-2, -2));
    }

    // --- dirty_chunks ---

    #[test]
    fn progress_generation_marks_dirty_chunks() {
        let mut world = World::new();
        // Blinker at origin
        world.set_alive(0, -1, true);
        world.set_alive(0, 0, true);
        world.set_alive(0, 1, true);
        world.clear_dirty_chunks();

        world.progress_generation();

        // 変化があるので dirty_chunks は空でない
        assert!(!world.dirty_chunks().is_empty());
    }

    #[test]
    fn clear_dirty_chunks_works() {
        let mut world = World::new();
        world.toggle_cell(0, 0);
        assert!(!world.dirty_chunks().is_empty());
        world.clear_dirty_chunks();
        assert!(world.dirty_chunks().is_empty());
    }

    // --- 負の座標でのシミュレーション ---

    #[test]
    fn blinker_at_negative_coordinates() {
        let mut world = World::new();
        world.set_alive(-1, 0, true);
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);

        world.progress_generation();
        assert!(world.is_alive(0, -1));
        assert!(world.is_alive(0, 0));
        assert!(world.is_alive(0, 1));
        assert!(!world.is_alive(-1, 0));
        assert!(!world.is_alive(1, 0));
    }

    #[test]
    fn pattern_across_chunk_boundary() {
        // チャンク境界(31,32)をまたぐblinker
        let mut world = World::new();
        world.set_alive(31, 0, true);
        world.set_alive(32, 0, true);
        world.set_alive(33, 0, true);

        world.progress_generation();
        assert!(world.is_alive(32, -1));
        assert!(world.is_alive(32, 0));
        assert!(world.is_alive(32, 1));
    }

    #[test]
    fn place_pattern_sets_cells_alive() {
        let mut world = World::new();
        let cells = &[(0, 0), (1, 0), (0, 1)];
        world.place_pattern(cells);
        assert!(world.is_alive(0, 0));
        assert!(world.is_alive(1, 0));
        assert!(world.is_alive(0, 1));
        assert!(!world.is_alive(1, 1));
    }

    #[test]
    fn place_pattern_records_initial_cells() {
        let mut world = World::new();
        let cells = &[(0, 0), (1, 0)];
        world.place_pattern(cells);
        assert!(world.is_initial_alive(0, 0));
        assert!(world.is_initial_alive(1, 0));
    }

    #[test]
    fn place_pattern_resets_generation_count() {
        let mut world = World::new();
        world.toggle_cell(0, 0);
        world.clear_dirty_chunks();
        world.progress_generation();
        assert!(world.generation_count > 0);
        world.place_pattern(&[(5, 5)]);
        assert_eq!(world.generation_count, 0);
    }

    #[test]
    fn place_pattern_marks_dirty_chunks() {
        let mut world = World::new();
        world.clear_dirty_chunks();
        world.place_pattern(&[(0, 0), (100, 100)]);
        let dirty = world.dirty_chunks();
        assert!(dirty.contains(&World::chunk_key(0, 0)));
        assert!(dirty.contains(&World::chunk_key(100, 100)));
    }
}
