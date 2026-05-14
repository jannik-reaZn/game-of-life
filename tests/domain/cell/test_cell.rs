use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;

use rstest::rstest;

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
    let cell = Cell::new(State::Live, Cell::create_neighbours(1));

    // THEN
    assert_eq!(cell.state(), State::Live);
    assert_eq!(cell.neighbours().len(), 8);
}

#[test]
fn test_create_neighbours_factory() {
    // GIVEN
    let neighbours = Cell::create_neighbours(3);

    // THEN
    assert_eq!(neighbours.len(), 8);
    assert_eq!(
        neighbours
            .iter()
            .filter(|neighbour| neighbour.state() == State::Live)
            .count(),
        3
    );
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

#[rstest]
#[case(1)]
#[case(4)]
#[case(8)]
fn test_count_living_neighbour_cell(#[case] living_cells: usize) {
    // GIVEN
    let neighbours = Cell::create_neighbours(living_cells);
    let cell = Cell::new(State::Live, neighbours);

    // WHEN
    let counted_living_cells = cell.count_living_neighbour_cells();

    // THEN
    assert_eq!(counted_living_cells, living_cells)
}
