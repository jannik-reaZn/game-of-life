use crate::domain::cell::Cell;
use crate::domain::position::Position;
use crate::domain::state::State;

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

    pub fn get_living_neighbours(&self, position: Position) -> usize {
        self.get_neighbour_cells(position)
            .iter()
            .filter(|cell| cell.state() == State::Live)
            .count()
    }

    pub fn get_neighbour_cells(&self, position: Position) -> Vec<Cell> {
        let directions = [-1isize, 0, 1];
        let row = position.row();
        let col = position.col();

        let mut neighbours: Vec<Cell> = Vec::new();
        for row_dir in &directions {
            for col_dir in &directions {
                // Center cell is not a neighbour
                if *row_dir == 0 && *col_dir == 0 {
                    continue;
                }

                let new_row = row as isize + row_dir;
                let new_col = col as isize + col_dir;

                // Check for out of bounds
                if new_row < 0 || new_col < 0 {
                    continue;
                }

                let new_row = new_row as usize;
                if new_row >= self.cells.len() {
                    continue;
                }

                let new_col = new_col as usize;
                if new_col >= self.cells[new_row].len() {
                    continue;
                }

                // Valid neighbours can be added to list
                neighbours.push(self.cells[new_row][new_col].clone());
            }
        }

        neighbours
    }
}
