use crate::domain::cell::Cell;

pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(cells: Vec<Vec<Cell>>) -> Self {
        Board { cells }
    }

    pub fn cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut Vec<Vec<Cell>> {
        &mut self.cells
    }
}
