use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::should_survive;
use game_of_life::domain::state::State;
use rstest::rstest;

#[rstest]
#[case(2)]
#[case(3)]
fn test_living_cell_has_survived(#[case] living_cells: usize) {
    // GIVEN
    let cell = Cell::new(State::Live, Cell::create_neighbours(living_cells));

    // WHEN
    let has_survived = should_survive(&cell);

    // THEN
    assert_eq!(has_survived, true);
}

#[rstest]
#[case(1)]
#[case(4)]
fn test_living_cell_has_not_survived(#[case] living_cells: usize) {
    // GIVEN
    let cell = Cell::new(State::Live, Cell::create_neighbours(living_cells));

    // WHEN
    let has_survived = should_survive(&cell);

    // THEN
    assert_eq!(has_survived, false);
}

#[test]
fn test_dead_cell_unchanged() {
    // GIVEN
    let cell = Cell::new(State::Dead, Cell::create_neighbours(1));

    // WHEN
    let has_survived = should_survive(&cell);

    // THEN
    assert_eq!(has_survived, false);
}
