use super::state::State;

pub struct Cell {
    state: State,
    neighbours: Vec<Cell>
}

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

    pub fn neighbours(&self) -> &Vec<Cell> {
        &self.neighbours
    }
}