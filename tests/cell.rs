use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;

#[test]
fn test_cell_state() {
    let cell = Cell::new(State::Dead, Vec::new());

    assert_eq!(cell.state(), State::Dead);
    assert!(cell.neighbours().is_empty());
}


#[test]
fn test_cell_neighbours() {
    let neighbour = Cell::new(State::Live, Vec::new());
    let cell = Cell::new(State::Live, vec![neighbour]);

    assert_eq!(cell.state(), State::Live);
    assert_eq!(cell.neighbours().len(), 1);
}