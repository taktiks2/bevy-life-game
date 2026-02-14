//! ライフゲームのワールド（セルグリッド）リソース

use bevy::prelude::Resource;

use super::simulation;

/// ライフゲームのワールドを表すリソース
///
/// セルの生死状態をフラット配列 (`Vec<bool>`) で管理する。
/// ダブルバッファリングにより、世代更新時のアロケーションを回避する。
/// `initial_cells` にユーザーが配置した初期パターンを保存し、リセット時に復元できる。
#[derive(Resource, Clone, Debug)]
pub struct World {
    /// 現在のセル状態（row-major: y * width + x）
    cells: Vec<bool>,
    /// 世代更新時の書き込み先バッファ（swap で交互に使用）
    back_buffer: Vec<bool>,
    /// ユーザーが配置した初期パターン（リセット時に復元）
    initial_cells: Vec<bool>,
    /// ワールドの幅（セル数）
    pub width: u16,
    /// ワールドの高さ（セル数）
    pub height: u16,
    /// 現在の世代数
    pub generation_count: u64,
}

impl World {
    /// 全セルが死んだ状態の新しいワールドを生成する
    pub fn new(width: u16, height: u16) -> Self {
        let size = width as usize * height as usize;
        Self {
            cells: vec![false; size],
            back_buffer: vec![false; size],
            initial_cells: vec![false; size],
            width,
            height,
            generation_count: 0,
        }
    }
    /// (x, y) 座標をフラット配列のインデックスに変換する
    fn idx(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }
    /// 指定座標のセルが生きているかを返す
    pub fn is_alive(&self, x: u16, y: u16) -> bool {
        self.cells[self.idx(x, y)]
    }
    /// コンウェイのルールに従い世代を1つ進める
    ///
    /// バックバッファに次世代の状態を計算し、swap で切り替える。
    pub fn progress_generation(&mut self) {
        self.generation_count += 1;
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let neighbors =
                    simulation::count_alive_neighbors(&self.cells, self.width, self.height, y, x);
                self.back_buffer[y * self.width as usize + x] =
                    simulation::next_cell_state(self.cells[y * self.width as usize + x], neighbors);
            }
        }
        std::mem::swap(&mut self.cells, &mut self.back_buffer);
    }
    /// 指定座標のセルの生死をトグルする
    ///
    /// 初期パターンも同時に更新し、世代カウントを0にリセットする。
    pub fn toggle_cell(&mut self, x: u16, y: u16) {
        let i = self.idx(x, y);
        let new_state = !self.cells[i];
        self.cells[i] = new_state;
        self.initial_cells[i] = new_state;
        self.generation_count = 0;
    }
    /// 初期パターンの状態に復元し、世代カウントを0にリセットする
    pub fn reset(&mut self) {
        self.cells = self.initial_cells.clone();
        self.generation_count = 0;
    }
    /// 全セルを死んだ状態にし、初期パターンもクリアする
    pub fn clear(&mut self) {
        let size = self.width as usize * self.height as usize;
        self.cells = vec![false; size];
        self.back_buffer = vec![false; size];
        self.initial_cells = vec![false; size];
        self.generation_count = 0;
    }
}

#[cfg(test)]
impl World {
    /// 指定座標のセルの生死状態を設定する（テスト専用）
    pub fn set_alive(&mut self, x: u16, y: u16, alive: bool) {
        let i = self.idx(x, y);
        self.cells[i] = alive;
    }
    /// 初期パターンにおいて指定座標のセルが生きているかを返す（テスト専用）
    pub fn is_initial_alive(&self, x: u16, y: u16) -> bool {
        self.initial_cells[self.idx(x, y)]
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
    fn new_world_has_all_dead_cells() {
        let world = World::new(5, 5);
        for y in 0..5u16 {
            for x in 0..5u16 {
                assert!(!world.is_alive(x, y));
            }
        }
    }

    #[test]
    fn new_world_has_correct_dimensions() {
        let world = World::new(10, 7);
        assert_eq!(world.width, 10);
        assert_eq!(world.height, 7);
    }

    #[test]
    fn new_world_has_zero_generation() {
        let world = World::new(5, 5);
        assert_eq!(world.generation_count, 0);
    }

    // --- toggle_cell (switch_state の代替テスト含む) ---

    #[test]
    fn toggle_cell_switches_dead_to_alive() {
        let mut world = World::new(3, 3);
        world.toggle_cell(1, 1);
        assert!(world.is_alive(1, 1));
    }

    #[test]
    fn toggle_cell_switches_alive_to_dead() {
        let mut world = World::new(3, 3);
        world.set_alive(1, 1, true);
        world.toggle_cell(1, 1);
        assert!(!world.is_alive(1, 1));
    }

    #[test]
    fn toggle_cell_syncs_initial_cells() {
        let mut world = World::new(3, 3);
        world.toggle_cell(1, 1);
        assert!(world.is_initial_alive(1, 1));
    }

    #[test]
    fn toggle_cell_resets_generation_count() {
        let mut world = World::new(3, 3);
        world.generation_count = 5;
        world.toggle_cell(1, 1);
        assert_eq!(world.generation_count, 0);
    }

    #[test]
    fn toggle_cell_does_not_clone_entire_grid() {
        let mut world = World::new(3, 3);
        world.set_alive(0, 0, true);
        world.save_as_initial();

        world.toggle_cell(1, 1);

        // 他のセルの initial_cells は変わらない
        assert!(world.is_initial_alive(0, 0));
        // トグルしたセルだけ同期される
        assert!(world.is_initial_alive(1, 1));
    }

    // --- Conway's ルール (progress_generation 経由) ---

    #[test]
    fn alive_cell_with_0_neighbors_dies() {
        let mut world = World::new(3, 3);
        world.set_alive(1, 1, true);
        world.progress_generation();
        assert!(!world.is_alive(1, 1));
    }

    #[test]
    fn alive_cell_with_1_neighbor_dies() {
        let mut world = World::new(3, 3);
        world.set_alive(1, 1, true);
        world.set_alive(0, 0, true);
        world.progress_generation();
        assert!(!world.is_alive(1, 1));
    }

    #[test]
    fn alive_cell_with_2_neighbors_survives() {
        let mut world = World::new(3, 3);
        world.set_alive(1, 1, true);
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.progress_generation();
        assert!(world.is_alive(1, 1));
    }

    #[test]
    fn alive_cell_with_3_neighbors_survives() {
        let mut world = World::new(3, 3);
        world.set_alive(1, 1, true);
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.set_alive(2, 0, true);
        world.progress_generation();
        assert!(world.is_alive(1, 1));
    }

    #[test]
    fn alive_cell_with_4_neighbors_dies() {
        let mut world = World::new(3, 3);
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
        let mut world = World::new(3, 3);
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.set_alive(0, 1, true);
        world.progress_generation();
        assert!(world.is_alive(1, 1));
    }

    #[test]
    fn dead_cell_with_2_neighbors_stays_dead() {
        let mut world = World::new(3, 3);
        world.set_alive(0, 0, true);
        world.set_alive(1, 0, true);
        world.progress_generation();
        assert!(!world.is_alive(1, 1));
    }

    // --- 有名パターン ---

    #[test]
    fn blinker_oscillates() {
        // 横棒 → 縦棒
        let mut world = World::new(5, 5);
        world.set_alive(1, 2, true);
        world.set_alive(2, 2, true);
        world.set_alive(3, 2, true);

        world.progress_generation();
        // 縦棒になるはず
        assert!(!world.is_alive(1, 2));
        assert!(world.is_alive(2, 1));
        assert!(world.is_alive(2, 2));
        assert!(world.is_alive(2, 3));
        assert!(!world.is_alive(3, 2));

        world.progress_generation();
        // 横棒に戻るはず
        assert!(world.is_alive(1, 2));
        assert!(world.is_alive(2, 2));
        assert!(world.is_alive(3, 2));
        assert!(!world.is_alive(2, 1));
        assert!(!world.is_alive(2, 3));
    }

    #[test]
    fn block_is_stable() {
        let mut world = World::new(4, 4);
        world.set_alive(1, 1, true);
        world.set_alive(2, 1, true);
        world.set_alive(1, 2, true);
        world.set_alive(2, 2, true);

        world.progress_generation();
        assert!(world.is_alive(1, 1));
        assert!(world.is_alive(2, 1));
        assert!(world.is_alive(1, 2));
        assert!(world.is_alive(2, 2));
    }

    // --- generation_count ---

    #[test]
    fn generation_count_increments() {
        let mut world = World::new(3, 3);
        assert_eq!(world.generation_count, 0);
        world.progress_generation();
        assert_eq!(world.generation_count, 1);
        world.progress_generation();
        assert_eq!(world.generation_count, 2);
    }

    // --- reset ---

    #[test]
    fn reset_restores_initial_cells() {
        let mut world = World::new(3, 3);
        world.set_alive(0, 0, true);
        world.save_as_initial();

        world.progress_generation();
        world.reset();

        assert!(world.is_alive(0, 0));
        assert_eq!(world.generation_count, 0);
    }

    // --- clear ---

    #[test]
    fn clear_sets_all_cells_dead() {
        let mut world = World::new(3, 3);
        world.set_alive(0, 0, true);
        world.set_alive(1, 1, true);
        world.generation_count = 5;

        world.clear();

        for y in 0..3u16 {
            for x in 0..3u16 {
                assert!(!world.is_alive(x, y));
                assert!(!world.is_initial_alive(x, y));
            }
        }
        assert_eq!(world.generation_count, 0);
    }

    // --- 境界条件 ---

    #[test]
    fn corner_cell_counts_neighbors_correctly() {
        // 左上角(0,0)に3隣接 → 誕生
        let mut world = World::new(3, 3);
        world.set_alive(1, 0, true);
        world.set_alive(0, 1, true);
        world.set_alive(1, 1, true);

        world.progress_generation();
        assert!(world.is_alive(0, 0));
    }

    #[test]
    fn edge_cell_counts_neighbors_correctly() {
        // 上辺の中央(0,1)に隣接2つ → Dead のまま
        let mut world = World::new(3, 3);
        world.set_alive(0, 0, true);
        world.set_alive(2, 0, true);

        world.progress_generation();
        assert!(!world.is_alive(1, 0));
    }
}
