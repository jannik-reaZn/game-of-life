use super::cell::Cell;
use super::state::State;

/// A live cell with two or three live neighbours survives.
pub fn survive(cell: &Cell) -> bool {
    match cell.state() {
        State::Live => {
            let living_neighbour_cells = cell.count_living_neighbour_cells();
            if living_neighbour_cells == 2 || living_neighbour_cells == 3 {
                return true;
            }
            false
        }
        State::Dead => false,
    }
}

/// A dead cell with exactly three live neighbours becomes a live cell.
pub fn born(cell: &Cell) -> bool {
    match cell.state() {
        State::Live => false,
        State::Dead => {
            let living_neighbour_cells = cell.count_living_neighbour_cells();
            if living_neighbour_cells == 3 {
                return true;
            }
            false
        }
    }
}

/// A live cell with fewer than two or more than three live neighbours die
pub fn die(cell: &Cell) -> bool {
    match cell.state() {
        State::Live => {
            let living_neighbour_cells = cell.count_living_neighbour_cells();
            if living_neighbour_cells < 2 || living_neighbour_cells > 3 {
                return true;
            }
            false
        }
        State::Dead => false,
    }
}

// pub fn game_of_live_orchestrator(cells: Vec<Cell>) {
//     // 1. Iterate though vec of cells
//     for cell in cells {
//         // 2. Check state of cell
//         match cell.state() {
//             State::Live => handle_living_cell(cell),
//             State::Dead => handle_dead_cell(cell),
//         }

//     }
// }

// 2.1 If state is "Life", then either survive or die
pub fn handle_living_cell(cell: &mut Cell) -> &mut Cell {
    match cell.state() {
        State::Live => {
            if survive(cell) {
                cell.set_state(State::Live);
            } else if die(cell) {
                cell.set_state(State::Dead);
            }

            cell
        }
        State::Dead => cell,
    }
}

// 2.2 If state is "Dead", then either born or nothing
pub fn handle_dead_cell(cell: &mut Cell) -> &mut Cell {
    match cell.state() {
        State::Live => cell,
        State::Dead => {
            if born(cell) {
                cell.set_state(State::Live);
            }
            cell
        }
    }
}
