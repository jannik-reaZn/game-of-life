use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;

#[test]
fn test_cell_state() {
    // GIVEN
    let cell = Cell::new(State::Dead, Vec::new());

    // THEN
    assert_eq!(cell.state(), State::Dead);
    assert!(cell.neighbours().is_empty());
}


#[test]
fn test_cell_neighbours() {
    // GIVEN
    let neighbour = Cell::new(State::Live, Vec::new());
    let cell = Cell::new(State::Live, vec![neighbour]);

    // THEN
    assert_eq!(cell.state(), State::Live);
    assert_eq!(cell.neighbours().len(), 1);
}

#[test]
fn test_set_cell_state() {
    // GIVEN
    let mut cell = Cell::new(State::Live, Vec::new());
    assert_eq!(cell.state(), State::Live);

    // WHEN
    cell.set_state(State::Dead);

    // THEN
    assert_eq!(cell.state(), State::Dead);
}