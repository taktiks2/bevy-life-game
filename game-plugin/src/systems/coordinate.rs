//! グリッド座標とワールド空間座標の変換

use bevy::prelude::*;
use common::consts::{GRID_DISPLAY_SIZE, cell_size};

/// グリッド座標をワールド空間の座標に変換する
pub fn world_to_screen_pos(grid_x: u16, grid_y: u16, world_width: u16, world_height: u16) -> Vec2 {
    let (cell_w, cell_h) = cell_size(world_width, world_height);
    Vec2::new(
        grid_x as f32 * cell_w - GRID_DISPLAY_SIZE / 2.0 + cell_w / 2.0,
        -(grid_y as f32 * cell_h - GRID_DISPLAY_SIZE / 2.0 + cell_h / 2.0),
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
    let half = GRID_DISPLAY_SIZE / 2.0;

    let local_x = world_pos.x + half;
    let local_y = -world_pos.y + half;

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

/// カーソルがワールドビューポート内にあるかを判定する
///
/// ボトムパネル領域上のカーソルを弾くために使用。
/// `main_height` はワールドカメラのビューポート高さ（物理ピクセル）。
/// 物理ピクセル（Viewport定義）と論理ピクセル（cursor_position戻り値）の
/// スケールファクター変換を考慮する。
pub fn is_cursor_over_world_viewport(cursor_pos: Vec2, scale_factor: f32, main_height: u32) -> bool {
    let logical_world_height = (main_height as f32) / scale_factor;
    cursor_pos.y < logical_world_height
}

#[cfg(test)]
mod tests {
    use super::*;

    // GRID_DISPLAY_SIZE = 800, 100x100 grid
    // cell size = 8x8 pixels

    #[test]
    fn center_of_grid_returns_correct_coords() {
        let result = screen_to_grid_coords(Vec2::new(0.0, 0.0), 100, 100);
        assert_eq!(result, Some((50, 50)));
    }

    #[test]
    fn top_left_corner_returns_0_0() {
        let result = screen_to_grid_coords(Vec2::new(-400.0, 400.0), 100, 100);
        assert_eq!(result, Some((0, 0)));
    }

    #[test]
    fn bottom_right_returns_99_99() {
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

    #[test]
    fn cursor_in_world_viewport_with_scale_1() {
        // scale_factor=1.0, main_height=720 physical pixels
        // ワールド領域: Y=0..720 logical
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 0.), 1.0, 720));
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 360.), 1.0, 720));
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 719.), 1.0, 720));
    }

    #[test]
    fn cursor_in_panel_with_scale_1() {
        // パネル領域: Y>=720 logical
        assert!(!is_cursor_over_world_viewport(Vec2::new(500., 720.), 1.0, 720));
        assert!(!is_cursor_over_world_viewport(Vec2::new(500., 750.), 1.0, 720));
        assert!(!is_cursor_over_world_viewport(Vec2::new(500., 799.), 1.0, 720));
    }

    #[test]
    fn cursor_in_world_viewport_with_retina_scale() {
        // scale_factor=2.0, main_height=720 physical = 360 logical
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 0.), 2.0, 720));
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 180.), 2.0, 720));
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 359.), 2.0, 720));
    }

    #[test]
    fn cursor_in_panel_with_retina_scale() {
        // パネル領域: Y>=360 logical (scale=2.0)
        assert!(!is_cursor_over_world_viewport(Vec2::new(500., 360.), 2.0, 720));
        assert!(!is_cursor_over_world_viewport(Vec2::new(500., 380.), 2.0, 720));
        assert!(!is_cursor_over_world_viewport(Vec2::new(500., 399.), 2.0, 720));
    }

    #[test]
    fn viewport_boundary_exact() {
        // 境界値テスト: main_height=720, scale=1.0
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 719.9), 1.0, 720));
        assert!(!is_cursor_over_world_viewport(Vec2::new(500., 720.0), 1.0, 720));
    }

    #[test]
    fn cursor_viewport_with_different_window_sizes() {
        // 異なるウィンドウサイズでも正しく動作する
        // main_height=432 (480 * 0.9), scale=1.0
        assert!(is_cursor_over_world_viewport(Vec2::new(300., 431.), 1.0, 432));
        assert!(!is_cursor_over_world_viewport(Vec2::new(300., 432.), 1.0, 432));

        // main_height=972 (1080 * 0.9), scale=1.0
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 971.), 1.0, 972));
        assert!(!is_cursor_over_world_viewport(Vec2::new(500., 972.), 1.0, 972));
    }
}
