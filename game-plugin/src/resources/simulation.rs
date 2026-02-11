use common::consts::SQUARE_COORDINATES;

use super::world::Cell;

pub fn count_alive_neighbors(
    cells: &[Vec<Cell>],
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
                && matches!(
                    cells[target_abs_y as usize][target_abs_x as usize],
                    Cell::Alive
                )
        })
        .count()
}

pub fn next_cell_state(cell: &Cell, alive_neighbor_count: usize) -> Cell {
    match cell {
        Cell::Alive if alive_neighbor_count == 2 || alive_neighbor_count == 3 => Cell::Alive,
        Cell::Dead if alive_neighbor_count == 3 => Cell::Alive,
        _ => Cell::Dead,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_neighbors_center_all_alive() {
        let cells = vec![
            vec![Cell::Alive, Cell::Alive, Cell::Alive],
            vec![Cell::Alive, Cell::Dead, Cell::Alive],
            vec![Cell::Alive, Cell::Alive, Cell::Alive],
        ];
        assert_eq!(count_alive_neighbors(&cells, 3, 3, 1, 1), 8);
    }

    #[test]
    fn count_neighbors_corner() {
        let cells = vec![
            vec![Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
        ];
        // (0,0) の隣接: (0,1)=Alive, (1,0)=Alive, (1,1)=Alive
        assert_eq!(count_alive_neighbors(&cells, 3, 3, 0, 0), 3);
    }

    #[test]
    fn count_neighbors_edge() {
        let cells = vec![
            vec![Cell::Alive, Cell::Dead, Cell::Alive],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
        ];
        // (0,1) の隣接: (0,0)=Alive, (0,2)=Alive, 他はDead
        assert_eq!(count_alive_neighbors(&cells, 3, 3, 0, 1), 2);
    }

    #[test]
    fn next_state_alive_with_2_survives() {
        assert!(matches!(next_cell_state(&Cell::Alive, 2), Cell::Alive));
    }

    #[test]
    fn next_state_alive_with_3_survives() {
        assert!(matches!(next_cell_state(&Cell::Alive, 3), Cell::Alive));
    }

    #[test]
    fn next_state_alive_with_1_dies() {
        assert!(matches!(next_cell_state(&Cell::Alive, 1), Cell::Dead));
    }

    #[test]
    fn next_state_alive_with_4_dies() {
        assert!(matches!(next_cell_state(&Cell::Alive, 4), Cell::Dead));
    }

    #[test]
    fn next_state_dead_with_3_becomes_alive() {
        assert!(matches!(next_cell_state(&Cell::Dead, 3), Cell::Alive));
    }

    #[test]
    fn next_state_dead_with_2_stays_dead() {
        assert!(matches!(next_cell_state(&Cell::Dead, 2), Cell::Dead));
    }
}
