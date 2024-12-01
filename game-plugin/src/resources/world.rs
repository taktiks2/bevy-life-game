use bevy::{
    color::palettes::css,
    prelude::{Color, Resource},
};

use common::consts::SQUARE_COORDINATES;

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
    pub width: u16,
    pub height: u16,
}

impl World {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            cells: (0..height)
                .map(|_| (0..width).map(|_| Cell::Dead).collect())
                .collect(),
            width,
            height,
        }
    }
    pub fn progress_generation(&mut self) {
        let current_world = self.clone();
        for (abs_y, row) in current_world.cells.iter().enumerate() {
            for (abs_x, cell) in row.iter().enumerate() {
                let alive_neighbor_count = self.count_alive_neighbors(&current_world, abs_y, abs_x);
                self.evolve(abs_y, abs_x, cell, alive_neighbor_count);
            }
        }
    }
    fn count_alive_neighbors(&self, world: &World, abs_y: usize, abs_x: usize) -> usize {
        SQUARE_COORDINATES
            .iter()
            .filter(|(rel_y, rel_x)| {
                let target_abs_y = abs_y as i8 + rel_y;
                let target_abs_x = abs_x as i8 + rel_x;
                if target_abs_y >= 0
                    && target_abs_y < world.height as i8
                    && target_abs_x >= 0
                    && target_abs_x < world.width as i8
                {
                    matches!(
                        world.cells[target_abs_y as usize][target_abs_x as usize],
                        Cell::Alive
                    )
                } else {
                    false
                }
            })
            .count()
    }
    fn evolve(&mut self, abs_y: usize, abs_x: usize, cell: &Cell, alive_neighbor_count: usize) {
        self.cells[abs_y][abs_x] = match cell {
            Cell::Alive => {
                if alive_neighbor_count <= 1 || alive_neighbor_count >= 4 {
                    Cell::Dead
                } else {
                    Cell::Alive
                }
            }
            Cell::Dead => {
                if alive_neighbor_count == 3 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }
        }
    }
}
