use bevy::{
    color::palettes::css,
    prelude::{Color, Resource},
};

use super::simulation;

#[derive(Clone, Debug)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn get_color(&self) -> Color {
        match self {
            Cell::Alive => css::BLACK.into(),
            Cell::Dead => css::WHITE.into(),
        }
    }
    pub fn switch_state(&self) -> Cell {
        match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct World {
    pub cells: Vec<Vec<Cell>>,
    back_buffer: Vec<Vec<Cell>>,
    pub initial_cells: Vec<Vec<Cell>>,
    pub width: u16,
    pub height: u16,
    pub generation_count: u64,
}

impl World {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            cells: Self::init_cells(width, height),
            back_buffer: Self::init_cells(width, height),
            initial_cells: Self::init_cells(width, height),
            width,
            height,
            generation_count: 0,
        }
    }
    fn init_cells(width: u16, height: u16) -> Vec<Vec<Cell>> {
        (0..height)
            .map(|_| (0..width).map(|_| Cell::Dead).collect())
            .collect()
    }
    pub fn progress_generation(&mut self) {
        self.generation_count += 1;
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let neighbors =
                    simulation::count_alive_neighbors(&self.cells, self.width, self.height, y, x);
                self.back_buffer[y][x] =
                    simulation::next_cell_state(&self.cells[y][x], neighbors);
            }
        }
        std::mem::swap(&mut self.cells, &mut self.back_buffer);
    }
    pub fn toggle_cell(&mut self, x: u16, y: u16) {
        let cell = self.cells[y as usize][x as usize].switch_state();
        self.cells[y as usize][x as usize] = cell.clone();
        self.initial_cells[y as usize][x as usize] = cell;
        self.generation_count = 0;
    }
    pub fn reset(&mut self) {
        self.cells = self.initial_cells.clone();
        self.generation_count = 0;
    }
    pub fn clear(&mut self) {
        self.cells = Self::init_cells(self.width, self.height);
        self.back_buffer = Self::init_cells(self.width, self.height);
        self.initial_cells = Self::init_cells(self.width, self.height);
        self.generation_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_alive(cell: &Cell) -> bool {
        matches!(cell, Cell::Alive)
    }

    fn is_dead(cell: &Cell) -> bool {
        matches!(cell, Cell::Dead)
    }

    // --- World::new ---

    #[test]
    fn new_world_has_all_dead_cells() {
        let world = World::new(5, 5);
        for row in &world.cells {
            for cell in row {
                assert!(is_dead(cell));
            }
        }
    }

    #[test]
    fn new_world_has_correct_dimensions() {
        let world = World::new(10, 7);
        assert_eq!(world.width, 10);
        assert_eq!(world.height, 7);
        assert_eq!(world.cells.len(), 7);
        assert_eq!(world.cells[0].len(), 10);
    }

    #[test]
    fn new_world_has_zero_generation() {
        let world = World::new(5, 5);
        assert_eq!(world.generation_count, 0);
    }

    // --- Cell::switch_state ---

    #[test]
    fn switch_state_alive_to_dead() {
        let cell = Cell::Alive;
        assert!(is_dead(&cell.switch_state()));
    }

    #[test]
    fn switch_state_dead_to_alive() {
        let cell = Cell::Dead;
        assert!(is_alive(&cell.switch_state()));
    }

    // --- Conway's ルール (progress_generation 経由) ---

    #[test]
    fn alive_cell_with_0_neighbors_dies() {
        let mut world = World::new(3, 3);
        world.cells[1][1] = Cell::Alive;
        world.progress_generation();
        assert!(is_dead(&world.cells[1][1]));
    }

    #[test]
    fn alive_cell_with_1_neighbor_dies() {
        let mut world = World::new(3, 3);
        world.cells[1][1] = Cell::Alive;
        world.cells[0][0] = Cell::Alive;
        world.progress_generation();
        assert!(is_dead(&world.cells[1][1]));
    }

    #[test]
    fn alive_cell_with_2_neighbors_survives() {
        let mut world = World::new(3, 3);
        world.cells[1][1] = Cell::Alive;
        world.cells[0][0] = Cell::Alive;
        world.cells[0][1] = Cell::Alive;
        world.progress_generation();
        assert!(is_alive(&world.cells[1][1]));
    }

    #[test]
    fn alive_cell_with_3_neighbors_survives() {
        let mut world = World::new(3, 3);
        world.cells[1][1] = Cell::Alive;
        world.cells[0][0] = Cell::Alive;
        world.cells[0][1] = Cell::Alive;
        world.cells[0][2] = Cell::Alive;
        world.progress_generation();
        assert!(is_alive(&world.cells[1][1]));
    }

    #[test]
    fn alive_cell_with_4_neighbors_dies() {
        let mut world = World::new(3, 3);
        world.cells[1][1] = Cell::Alive;
        world.cells[0][0] = Cell::Alive;
        world.cells[0][1] = Cell::Alive;
        world.cells[0][2] = Cell::Alive;
        world.cells[1][0] = Cell::Alive;
        world.progress_generation();
        assert!(is_dead(&world.cells[1][1]));
    }

    #[test]
    fn dead_cell_with_3_neighbors_becomes_alive() {
        let mut world = World::new(3, 3);
        world.cells[0][0] = Cell::Alive;
        world.cells[0][1] = Cell::Alive;
        world.cells[1][0] = Cell::Alive;
        world.progress_generation();
        assert!(is_alive(&world.cells[1][1]));
    }

    #[test]
    fn dead_cell_with_2_neighbors_stays_dead() {
        let mut world = World::new(3, 3);
        world.cells[0][0] = Cell::Alive;
        world.cells[0][1] = Cell::Alive;
        world.progress_generation();
        assert!(is_dead(&world.cells[1][1]));
    }

    // --- 有名パターン ---

    #[test]
    fn blinker_oscillates() {
        // 横棒 → 縦棒
        let mut world = World::new(5, 5);
        world.cells[2][1] = Cell::Alive;
        world.cells[2][2] = Cell::Alive;
        world.cells[2][3] = Cell::Alive;

        world.progress_generation();
        // 縦棒になるはず
        assert!(is_dead(&world.cells[2][1]));
        assert!(is_alive(&world.cells[1][2]));
        assert!(is_alive(&world.cells[2][2]));
        assert!(is_alive(&world.cells[3][2]));
        assert!(is_dead(&world.cells[2][3]));

        world.progress_generation();
        // 横棒に戻るはず
        assert!(is_alive(&world.cells[2][1]));
        assert!(is_alive(&world.cells[2][2]));
        assert!(is_alive(&world.cells[2][3]));
        assert!(is_dead(&world.cells[1][2]));
        assert!(is_dead(&world.cells[3][2]));
    }

    #[test]
    fn block_is_stable() {
        let mut world = World::new(4, 4);
        world.cells[1][1] = Cell::Alive;
        world.cells[1][2] = Cell::Alive;
        world.cells[2][1] = Cell::Alive;
        world.cells[2][2] = Cell::Alive;

        world.progress_generation();
        assert!(is_alive(&world.cells[1][1]));
        assert!(is_alive(&world.cells[1][2]));
        assert!(is_alive(&world.cells[2][1]));
        assert!(is_alive(&world.cells[2][2]));
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
        world.cells[0][0] = Cell::Alive;
        world.initial_cells = world.cells.clone();

        world.progress_generation();
        world.reset();

        assert!(is_alive(&world.cells[0][0]));
        assert_eq!(world.generation_count, 0);
    }

    // --- clear ---

    #[test]
    fn clear_sets_all_cells_dead() {
        let mut world = World::new(3, 3);
        world.cells[0][0] = Cell::Alive;
        world.cells[1][1] = Cell::Alive;
        world.generation_count = 5;

        world.clear();

        for row in &world.cells {
            for cell in row {
                assert!(is_dead(cell));
            }
        }
        for row in &world.initial_cells {
            for cell in row {
                assert!(is_dead(cell));
            }
        }
        assert_eq!(world.generation_count, 0);
    }

    // --- toggle_cell ---

    #[test]
    fn toggle_cell_switches_dead_to_alive() {
        let mut world = World::new(3, 3);
        world.toggle_cell(1, 1);
        assert!(is_alive(&world.cells[1][1]));
    }

    #[test]
    fn toggle_cell_switches_alive_to_dead() {
        let mut world = World::new(3, 3);
        world.cells[1][1] = Cell::Alive;
        world.toggle_cell(1, 1);
        assert!(is_dead(&world.cells[1][1]));
    }

    #[test]
    fn toggle_cell_syncs_initial_cells() {
        let mut world = World::new(3, 3);
        world.toggle_cell(1, 1);
        assert!(is_alive(&world.initial_cells[1][1]));
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
        world.cells[0][0] = Cell::Alive;
        world.initial_cells = world.cells.clone();

        world.toggle_cell(1, 1);

        // 他のセルの initial_cells は変わらない
        assert!(is_alive(&world.initial_cells[0][0]));
        // トグルしたセルだけ同期される
        assert!(is_alive(&world.initial_cells[1][1]));
    }

    // --- 境界条件 ---

    #[test]
    fn corner_cell_counts_neighbors_correctly() {
        // 左上角(0,0)に3隣接 → 誕生
        let mut world = World::new(3, 3);
        world.cells[0][1] = Cell::Alive;
        world.cells[1][0] = Cell::Alive;
        world.cells[1][1] = Cell::Alive;

        world.progress_generation();
        assert!(is_alive(&world.cells[0][0]));
    }

    #[test]
    fn edge_cell_counts_neighbors_correctly() {
        // 上辺の中央(0,1)に隣接2つ → Dead のまま
        let mut world = World::new(3, 3);
        world.cells[0][0] = Cell::Alive;
        world.cells[0][2] = Cell::Alive;

        world.progress_generation();
        assert!(is_dead(&world.cells[0][1]));
    }
}
