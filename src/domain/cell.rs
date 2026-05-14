use super::state::State;

pub struct Cell {
    state: State,
    neighbours: Vec<Cell>
}

const MAX_NEIGHBOUR_CELLS: usize = 8;

impl Cell {
    pub fn new(state: State, neighbours: Vec<Cell>) -> Self {
        Self {
            state,
            neighbours
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn neighbours(&self) -> &Vec<Cell> {
        &self.neighbours
    }

    pub fn create_neighbours(living_cells: usize) -> Vec<Cell> {
        if living_cells > MAX_NEIGHBOUR_CELLS {
            return Vec::new();
        }

        let mut neighbours = Vec::with_capacity(MAX_NEIGHBOUR_CELLS);

        for _ in 0..living_cells {
            neighbours.push(Cell::new(State::Live, Vec::new()));
        }

        for _ in living_cells..MAX_NEIGHBOUR_CELLS {
            neighbours.push(Cell::new(State::Dead, Vec::new()));
        }

        neighbours
    }
}