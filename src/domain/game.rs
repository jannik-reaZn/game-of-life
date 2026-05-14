use crate::domain::board::Board;
use crate::domain::cell::Cell;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(cells: Vec<Vec<Cell>>) -> Self {
        Game {
            board: Board::new(cells),
        }
    }

    pub fn run_one_generation(&mut self) {
        self.board = self.board.next_generation();
    }

    pub fn cells(&self) -> &Vec<Vec<Cell>> {
        self.board.cells()
    }
}
