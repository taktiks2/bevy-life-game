//! コンウェイのライフゲームのシミュレーションロジック（純粋関数）
//!
//! Bevyに依存しない純粋な計算ロジックを提供する。

use common::consts::SQUARE_COORDINATES;

/// 指定座標の周囲8マス（ムーア近傍）の生存セル数を数える
///
/// グリッド境界外の座標は無視される（トーラスではない）。
pub fn count_alive_neighbors(
    cells: &[bool],
    width: u16,
    height: u16,
    abs_y: usize,
    abs_x: usize,
) -> usize {
    SQUARE_COORDINATES
        .iter()
        .filter(|(rel_y, rel_x)| {
            let target_abs_y = abs_y as i8 + rel_y;
            let target_abs_x = abs_x as i8 + rel_x;
            target_abs_y >= 0
                && target_abs_y < height as i8
                && target_abs_x >= 0
                && target_abs_x < width as i8
                && cells[target_abs_y as usize * width as usize + target_abs_x as usize]
        })
        .count()
}

/// コンウェイのルールに基づき次世代のセル状態を決定する
///
/// - 生存セル: 隣接2-3で生存、それ以外は死亡（過疎/過密）
/// - 死亡セル: 隣接ちょうど3で誕生
pub fn next_cell_state(alive: bool, alive_neighbor_count: usize) -> bool {
    matches!((alive, alive_neighbor_count), (true, 2) | (true, 3) | (false, 3))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_grid(width: u16, height: u16, alive_coords: &[(u16, u16)]) -> Vec<bool> {
        let mut cells = vec![false; width as usize * height as usize];
        for &(x, y) in alive_coords {
            cells[y as usize * width as usize + x as usize] = true;
        }
        cells
    }

    #[test]
    fn count_neighbors_center_all_alive() {
        let cells = make_grid(3, 3, &[
            (0, 0), (1, 0), (2, 0),
            (0, 1),         (2, 1),
            (0, 2), (1, 2), (2, 2),
        ]);
        assert_eq!(count_alive_neighbors(&cells, 3, 3, 1, 1), 8);
    }

    #[test]
    fn count_neighbors_corner() {
        let cells = make_grid(3, 3, &[(1, 0), (0, 1), (1, 1)]);
        // (0,0) の隣接: (1,0)=Alive, (0,1)=Alive, (1,1)=Alive
        assert_eq!(count_alive_neighbors(&cells, 3, 3, 0, 0), 3);
    }

    #[test]
    fn count_neighbors_edge() {
        let cells = make_grid(3, 3, &[(0, 0), (2, 0)]);
        // (0,1) の隣接: (0,0)=Alive, (2,0)=Alive, 他はDead
        assert_eq!(count_alive_neighbors(&cells, 3, 3, 0, 1), 2);
    }

    #[test]
    fn next_state_alive_with_2_survives() {
        assert!(next_cell_state(true, 2));
    }

    #[test]
    fn next_state_alive_with_3_survives() {
        assert!(next_cell_state(true, 3));
    }

    #[test]
    fn next_state_alive_with_1_dies() {
        assert!(!next_cell_state(true, 1));
    }

    #[test]
    fn next_state_alive_with_4_dies() {
        assert!(!next_cell_state(true, 4));
    }

    #[test]
    fn next_state_dead_with_3_becomes_alive() {
        assert!(next_cell_state(false, 3));
    }

    #[test]
    fn next_state_dead_with_2_stays_dead() {
        assert!(!next_cell_state(false, 2));
    }
}
