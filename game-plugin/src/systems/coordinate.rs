//! グリッド座標とワールド空間座標の変換

use bevy::prelude::*;
use common::consts::{MAIN_PHYSICAL_WIDTH, WINDOW_HEIGHT, cell_size};

/// グリッド座標をワールド空間の座標に変換する
pub fn world_to_screen_pos(grid_x: u16, grid_y: u16, world_width: u16, world_height: u16) -> Vec2 {
    let (cell_w, cell_h) = cell_size(world_width, world_height);
    Vec2::new(
        grid_x as f32 * cell_w - MAIN_PHYSICAL_WIDTH as f32 / 2.0 + cell_w / 2.0,
        -(grid_y as f32 * cell_h - WINDOW_HEIGHT / 2.0 + cell_h / 2.0),
    )
}

/// ワールド空間の座標をグリッド座標に変換する
///
/// グリッド範囲外の場合は `None` を返す。
pub fn screen_to_grid_coords(
    world_pos: Vec2,
    world_width: u16,
    world_height: u16,
) -> Option<(u16, u16)> {
    let half_w = MAIN_PHYSICAL_WIDTH as f32 / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;

    let local_x = world_pos.x + half_w;
    let local_y = -world_pos.y + half_h;

    if local_x < 0.0 || local_y < 0.0 {
        return None;
    }

    let (cell_w, cell_h) = cell_size(world_width, world_height);

    let grid_x = (local_x / cell_w) as u16;
    let grid_y = (local_y / cell_h) as u16;

    if grid_x >= world_width || grid_y >= world_height {
        return None;
    }

    Some((grid_x, grid_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    // MAIN_PHYSICAL_WIDTH = 800, WINDOW_HEIGHT = 800, 100x100 grid
    // cell size = 8x8 pixels

    #[test]
    fn center_of_grid_returns_correct_coords() {
        // World pos (0, 0) = center of grid = cell (50, 50) approximately
        let result = screen_to_grid_coords(Vec2::new(0.0, 0.0), 100, 100);
        assert_eq!(result, Some((50, 50)));
    }

    #[test]
    fn top_left_corner_returns_0_0() {
        // Top-left: world pos (-400, 400)
        let result = screen_to_grid_coords(Vec2::new(-400.0, 400.0), 100, 100);
        assert_eq!(result, Some((0, 0)));
    }

    #[test]
    fn bottom_right_returns_99_99() {
        // Bottom-right: world pos (399, -399)
        let result = screen_to_grid_coords(Vec2::new(399.0, -399.0), 100, 100);
        assert_eq!(result, Some((99, 99)));
    }

    #[test]
    fn outside_grid_left_returns_none() {
        let result = screen_to_grid_coords(Vec2::new(-401.0, 0.0), 100, 100);
        assert_eq!(result, None);
    }

    #[test]
    fn outside_grid_right_returns_none() {
        let result = screen_to_grid_coords(Vec2::new(401.0, 0.0), 100, 100);
        assert_eq!(result, None);
    }

    #[test]
    fn outside_grid_top_returns_none() {
        let result = screen_to_grid_coords(Vec2::new(0.0, 401.0), 100, 100);
        assert_eq!(result, None);
    }

    #[test]
    fn outside_grid_bottom_returns_none() {
        let result = screen_to_grid_coords(Vec2::new(0.0, -401.0), 100, 100);
        assert_eq!(result, None);
    }

    #[test]
    fn world_to_screen_and_back_roundtrips() {
        for gx in [0u16, 25, 50, 75, 99] {
            for gy in [0u16, 25, 50, 75, 99] {
                let screen_pos = world_to_screen_pos(gx, gy, 100, 100);
                let result = screen_to_grid_coords(screen_pos, 100, 100);
                assert_eq!(result, Some((gx, gy)), "roundtrip failed for ({}, {})", gx, gy);
            }
        }
    }
}
