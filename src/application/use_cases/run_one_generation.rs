use crate::application::use_cases::handle_dead_cell_use_case::handle_dead_cell;
use crate::application::use_cases::handle_living_cell_use_case::handle_living_cell;
use crate::domain::cell::Cell;
use crate::domain::state::State;

pub fn run_one_generation(cells: &mut Vec<Vec<Cell>>) -> &mut Vec<Vec<Cell>> {
    for row in cells.iter_mut() {
        for cell in row.iter_mut() {
            match cell.state() {
                State::Live => {
                    handle_living_cell(cell);
                }
                State::Dead => {
                    handle_dead_cell(cell);
                }
            }
        }
    }

    cells
}
