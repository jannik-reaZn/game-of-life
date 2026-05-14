use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::born;
use game_of_life::domain::state::State;
use rstest::rstest;

#[test]
fn test_dead_cell_is_born() {
    // GIVEN
    let cell = Cell::new(State::Dead, Cell::create_neighbours(3));

    // WHEN
    let shoud_be_born = born(&cell);

    // THEN
    assert_eq!(shoud_be_born, true);
}

#[rstest]
#[case(2)]
#[case(4)]
fn test_dead_cell_stays_dead(#[case] living_cells: usize) {
    // GIVEN
    let cell = Cell::new(State::Dead, Cell::create_neighbours(living_cells));

    // WHEN
    let should_be_born = born(&cell);

    // THEN
    assert_eq!(should_be_born, false);
}

#[test]
fn test_living_cell_stays_alive() {
    // GIVEN
    let cell = Cell::new(State::Live, Cell::create_neighbours(1));

    // WHEN
    let should_be_born = born(&cell);

    // THEN
    assert_eq!(should_be_born, false);
}
