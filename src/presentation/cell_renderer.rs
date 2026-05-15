use crate::domain::state::State;

struct CellStateRenderer {
    state: State,
}

trait Renderable {
    fn render(&self) -> char;
}

impl Renderable for CellStateRenderer {
    fn render(&self) -> char {
        match self.state {
            State::Live => 'O',
            State::Dead => '.',
        }
    }
}
