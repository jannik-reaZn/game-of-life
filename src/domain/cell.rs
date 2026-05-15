use super::state::State;

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    state: State,
}

impl Cell {
    pub fn new(state: State) -> Self {
        Cell { state }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}
