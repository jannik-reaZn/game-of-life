use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::{born, die, handle_living_cell, survive};
use game_of_life::domain::state::State;
use rstest::*;

#[test]
fn test_survive_rule() {
    let cell = Cell::new(State::Dead, Cell::create_neighbours(3));

    assert_eq!(cell.neighbours().len(), 8);
    let has_survived = survive(&cell);
    assert_eq!(has_survived, true);
}

#[test]
fn test_born_rule() {
    let cell = Cell::new(State::Dead, Cell::create_neighbours(3));
    assert_eq!(born(cell), true);
}

#[test]
fn test_die_rule() {
    let cell = Cell::new(State::Dead, Cell::create_neighbours(1));
    assert_eq!(die(&cell), true);
}

#[rstest]
#[case(2)]
#[case(3)]
fn test_handle_living_cell(#[case] living_cells: usize) {
    // GIVEN
    let mut cell = Cell::new(State::Live, Cell::create_neighbours(living_cells));

    // WHEN
    handle_living_cell(&mut cell);

    // THEN
    assert_eq!(cell.state(), State::Live);
}

#[rstest]
#[case(0)]
#[case(1)]
#[case[4]]
#[case[8]]
fn test_handle_living_cell_bad(#[case] living_cells: usize) {
    // GIVEN
    let mut cell = Cell::new(State::Live, Cell::create_neighbours(living_cells));

    // WHEN
    handle_living_cell(&mut cell);

    // THEN
    assert_eq!(
        cell.neighbours()
            .iter()
            .filter(|neighbour| neighbour.state() == State::Live)
            .count(),
        living_cells
    );
}
