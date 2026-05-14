use crate::application::use_cases::run_one_generation::run_one_generation;
use crate::domain::cell::Cell;

struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(cells: Vec<Cell>) -> Self {
        Board { cells }
    }

    pub fn next_generation(&self) -> Self {
        let mut new_cells = self.cells.clone();
        run_one_generation(&mut new_cells);
        Board { cells: new_cells }
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }
}
