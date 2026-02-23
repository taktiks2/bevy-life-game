//! グリッド座標とワールド空間座標の変換

use bevy::prelude::*;
use common::consts::CELL_WORLD_SIZE;

/// グリッド座標をワールド空間の座標に変換する
///
/// セルの中心座標を返す。Y軸は反転（グリッドY+が画面下方向）。
pub fn world_to_screen_pos(grid_x: i32, grid_y: i32) -> Vec2 {
    Vec2::new(
        grid_x as f32 * CELL_WORLD_SIZE + CELL_WORLD_SIZE / 2.0,
        -(grid_y as f32 * CELL_WORLD_SIZE + CELL_WORLD_SIZE / 2.0),
    )
}

/// ワールド空間の座標をグリッド座標に変換する
///
/// 無限フィールドなので常に有効な座標を返す。
pub fn screen_to_grid_coords(world_pos: Vec2) -> (i32, i32) {
    let grid_x = (world_pos.x / CELL_WORLD_SIZE).floor() as i32;
    let grid_y = (-world_pos.y / CELL_WORLD_SIZE).floor() as i32;
    (grid_x, grid_y)
}

/// カーソルがワールドビューポート内にあるかを判定する
///
/// ボトムパネル領域上のカーソルを弾くために使用。
/// `main_height` はワールドカメラのビューポート高さ（物理ピクセル）。
/// 物理ピクセル（Viewport定義）と論理ピクセル（cursor_position戻り値）の
/// スケールファクター変換を考慮する。
pub fn is_cursor_over_world_viewport(
    cursor_pos: Vec2,
    scale_factor: f32,
    main_height: u32,
) -> bool {
    let logical_world_height = (main_height as f32) / scale_factor;
    cursor_pos.y < logical_world_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_maps_to_cell_0_0_center() {
        // セル(0,0)の中心は (0.5, -0.5)
        let pos = world_to_screen_pos(0, 0);
        assert_eq!(pos, Vec2::new(0.5, -0.5));
    }

    #[test]
    fn negative_cell_position() {
        let pos = world_to_screen_pos(-1, -1);
        assert_eq!(pos, Vec2::new(-0.5, 0.5));
    }

    #[test]
    fn screen_to_grid_at_origin() {
        // ワールド座標(0.5, -0.5)はセル(0, 0)
        let (gx, gy) = screen_to_grid_coords(Vec2::new(0.5, -0.5));
        assert_eq!((gx, gy), (0, 0));
    }

    #[test]
    fn screen_to_grid_negative() {
        // ワールド座標(-0.5, 0.5)はセル(-1, -1)
        let (gx, gy) = screen_to_grid_coords(Vec2::new(-0.5, 0.5));
        assert_eq!((gx, gy), (-1, -1));
    }

    #[test]
    fn screen_to_grid_exact_boundary() {
        // ワールド座標(0.0, 0.0)はセル(0, -1)
        // x=0.0 → floor(0.0/1.0) = 0
        // y=0.0 → floor(-0.0/1.0) = floor(-0.0) = 0... but -y = -0.0
        let (gx, gy) = screen_to_grid_coords(Vec2::new(0.0, 0.0));
        assert_eq!(gx, 0);
        assert_eq!(gy, 0);
    }

    #[test]
    fn roundtrip_positive_coordinates() {
        for gx in [0, 1, 10, 50, 100] {
            for gy in [0, 1, 10, 50, 100] {
                let screen_pos = world_to_screen_pos(gx, gy);
                let (rx, ry) = screen_to_grid_coords(screen_pos);
                assert_eq!((rx, ry), (gx, gy), "roundtrip failed for ({}, {})", gx, gy);
            }
        }
    }

    #[test]
    fn roundtrip_negative_coordinates() {
        for gx in [-100, -50, -10, -1] {
            for gy in [-100, -50, -10, -1] {
                let screen_pos = world_to_screen_pos(gx, gy);
                let (rx, ry) = screen_to_grid_coords(screen_pos);
                assert_eq!((rx, ry), (gx, gy), "roundtrip failed for ({}, {})", gx, gy);
            }
        }
    }

    #[test]
    fn cursor_in_world_viewport_with_scale_1() {
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 0.), 1.0, 720));
        assert!(is_cursor_over_world_viewport(
            Vec2::new(500., 719.),
            1.0,
            720
        ));
    }

    #[test]
    fn cursor_in_panel_with_scale_1() {
        assert!(!is_cursor_over_world_viewport(
            Vec2::new(500., 720.),
            1.0,
            720
        ));
    }

    #[test]
    fn cursor_in_world_viewport_with_retina_scale() {
        // scale_factor=2.0, main_height=720 physical = 360 logical
        assert!(is_cursor_over_world_viewport(Vec2::new(500., 0.), 2.0, 720));
        assert!(is_cursor_over_world_viewport(
            Vec2::new(500., 359.),
            2.0,
            720
        ));
    }

    #[test]
    fn cursor_in_panel_with_retina_scale() {
        assert!(!is_cursor_over_world_viewport(
            Vec2::new(500., 360.),
            2.0,
            720
        ));
    }

    #[test]
    fn viewport_boundary_exact() {
        assert!(is_cursor_over_world_viewport(
            Vec2::new(500., 719.9),
            1.0,
            720
        ));
        assert!(!is_cursor_over_world_viewport(
            Vec2::new(500., 720.0),
            1.0,
            720
        ));
    }
}
