use crate::domain::cell::Cell;
use crate::domain::rules::should_be_born;
use crate::domain::state::State;

// 2.2 If state is "Dead", then either born or nothing
pub fn handle_dead_cell<'a>(cell: &'a mut Cell, living_neighbour_cells: &'a usize) -> &'a mut Cell {
    match cell.state() {
        State::Live => cell,
        State::Dead => {
            if should_be_born(cell, living_neighbour_cells) {
                cell.set_state(State::Live);
            }
            cell
        }
    }
}
