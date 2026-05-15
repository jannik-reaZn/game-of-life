use super::cell::Cell;
use super::state::State;

/// A live cell with two or three live neighbours survives.
pub fn should_survive(cell: &Cell, living_neighbour_cells: &usize) -> bool {
    match cell.state() {
        State::Live => {
            if *living_neighbour_cells == 2 || *living_neighbour_cells == 3 {
                return true;
            }
            false
        }
        State::Dead => false,
    }
}

/// A dead cell with exactly three live neighbours becomes a live cell.
pub fn should_be_born(cell: &Cell, living_neighbour_cells: &usize) -> bool {
    match cell.state() {
        State::Live => false,
        State::Dead => {
            if *living_neighbour_cells == 3 {
                return true;
            }
            false
        }
    }
}

/// A live cell with fewer than two or more than three live neighbours should_die
pub fn should_die(cell: &Cell, living_neighbour_cells: &usize) -> bool {
    match cell.state() {
        State::Live => {
            if *living_neighbour_cells < 2 || *living_neighbour_cells > 3 {
                return true;
            }
            false
        }
        State::Dead => false,
    }
}
