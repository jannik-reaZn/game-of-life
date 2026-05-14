use crate::application::use_cases::run_one_generation::run_one_generation;
use crate::domain::cell::Cell;

pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(cells: Vec<Vec<Cell>>) -> Self {
        Board { cells }
    }

    pub fn next_generation(&self) -> Self {
        let mut new_cells = self.cells.clone();
        run_one_generation(&mut new_cells);
        Board { cells: new_cells }
    }

    pub fn cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }
}
