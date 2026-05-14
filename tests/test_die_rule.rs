use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::die;
use game_of_life::domain::state::State;
use rstest::rstest;

#[rstest]
#[case(1)]
#[case(4)]
fn test_living_cell_dies(#[case] living_cells: usize) {
    // GIVEN
    let cell = Cell::new(State::Live, Cell::create_neighbours(living_cells));

    // WHEN
    let has_died = die(&cell);

    // THEN
    assert_eq!(has_died, true);
}

#[rstest]
#[case(2)]
#[case(3)]
fn test_living_cell_stays_alive(#[case] living_cells: usize) {
    // GIVEN
    let cell = Cell::new(State::Live, Cell::create_neighbours(living_cells));

    // WHEN
    let has_died = die(&cell);

    // THEN
    assert_eq!(has_died, false);
}

#[test]
fn test_dead_cell_stays_dead() {
    // GIVEN
    let cell = Cell::new(State::Dead, Cell::create_neighbours(3));

    // WHEN
    let has_died = die(&cell);

    // THEN
    assert_eq!(has_died, false);
}
