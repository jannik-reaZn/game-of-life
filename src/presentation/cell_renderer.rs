use crate::domain::state::State;

pub struct CellStateRenderer {
    state: State,
}

pub trait Renderable {
    fn render(&self) -> char;
}

impl CellStateRenderer {
    pub fn new(state: State) -> Self {
        CellStateRenderer { state }
    }
}

impl Renderable for CellStateRenderer {
    fn render(&self) -> char {
        match self.state {
            State::Live => 'O',
            State::Dead => '.',
        }
    }
}
