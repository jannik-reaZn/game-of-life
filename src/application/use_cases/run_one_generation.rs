use crate::application::use_cases::handle_dead_cell_use_case::handle_dead_cell;
use crate::application::use_cases::handle_living_cell_use_case::handle_living_cell;
use crate::domain::board::Board;
use crate::domain::state::State;

pub fn run_one_generation(board: &mut Board) -> &mut Board {
    for row in board.cells_mut().iter_mut() {
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

    board
}
