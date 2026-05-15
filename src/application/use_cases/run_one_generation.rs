use crate::application::use_cases::handle_dead_cell_use_case::handle_dead_cell;
use crate::application::use_cases::handle_living_cell_use_case::handle_living_cell;
use crate::domain::board::Board;
use crate::domain::state::State;

pub fn run_one_generation(board: &mut Board) -> &mut Board {
    let current_generation = Board::new(board.cells().clone());
    let row_count = board.cells().len();

    for row in 0..row_count {
        let col_count = board.cells()[row].len();

        for col in 0..col_count {
            let living_neighbours = current_generation.get_living_neighbour_cell(row, col);
            let cell = &mut board.cells_mut()[row][col];

            match cell.state() {
                State::Live => {
                    handle_living_cell(cell, &living_neighbours);
                }
                State::Dead => {
                    handle_dead_cell(cell, &living_neighbours);
                }
            }
        }
    }

    board
}
