use crate::domain::cell::Cell;
use crate::domain::rules::{should_die, should_survive};
use crate::domain::state::State;

// 2.1 If state is "Life", then either survive or die
pub fn handle_living_cell(cell: &mut Cell) -> &mut Cell {
    match cell.state() {
        State::Live => {
            if should_survive(cell) {
                cell.set_state(State::Live);
            } else if should_die(cell) {
                cell.set_state(State::Dead);
            }

            cell
        }
        State::Dead => cell,
    }
}
