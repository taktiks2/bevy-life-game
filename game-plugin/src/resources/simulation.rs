//! コンウェイのライフゲームのシミュレーションロジック（純粋関数）
//!
//! Bevyに依存しない純粋な計算ロジックを提供する。

/// コンウェイのルールに基づき次世代のセル状態を決定する
///
/// - 生存セル: 隣接2-3で生存、それ以外は死亡（過疎/過密）
/// - 死亡セル: 隣接ちょうど3で誕生
pub fn next_cell_state(alive: bool, alive_neighbor_count: usize) -> bool {
    matches!(
        (alive, alive_neighbor_count),
        (true, 2) | (true, 3) | (false, 3)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
