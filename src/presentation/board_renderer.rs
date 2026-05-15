use crate::domain::board::Board;
use crate::domain::cell::Cell;
use crate::domain::state::State;

// Interface for rendering the board
pub trait BoardRenderer {
    fn seed(&mut self);
    fn render(&self) -> String;
}

// Implementation of BoardRenderer for terminal output
pub struct TerminalBoardRenderer {
    board: Board,
}

impl TerminalBoardRenderer {
    pub fn new() -> Self {
        TerminalBoardRenderer {
            board: Board::new(vec![]),
        }
    }
}

impl BoardRenderer for TerminalBoardRenderer {
    fn seed(&mut self) {
        self.board = Board::new(vec![vec![
            Cell::new(State::Dead, Cell::create_neighbours(0)),
            Cell::new(State::Live, Cell::create_neighbours(1)),
            Cell::new(State::Dead, Cell::create_neighbours(0)),
        ]]);
    }

    fn render(&self) -> String {
        self.board
            .cells()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell.is_alive() {
                        true => 'O',
                        false => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
