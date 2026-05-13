use super::state::State;

struct Cell {
    state: State,
    neighbours: Vec<Cell>
}