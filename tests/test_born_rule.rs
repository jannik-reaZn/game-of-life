use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::should_be_born;
use game_of_life::domain::state::State;
use rstest::rstest;

#[test]
fn test_dead_cell_is_born() {
    // GIVEN
    let cell = Cell::new(State::Dead);
    let living_neighbours = 3;

    // WHEN
    let shoud_be_born = should_be_born(&cell, &living_neighbours);

    // THEN
    assert_eq!(shoud_be_born, true);
}

#[rstest]
#[case(2)]
#[case(4)]
fn test_dead_cell_stays_dead(#[case] living_neighbours: usize) {
    // GIVEN
    let cell = Cell::new(State::Dead);

    // WHEN
    let should_be_born = should_be_born(&cell, &living_neighbours);

    // THEN
    assert_eq!(should_be_born, false);
}

#[test]
fn test_living_cell_stays_alive() {
    // GIVEN
    let cell = Cell::new(State::Live);
    let living_neighbours = 1;

    // WHEN
    let should_be_born = should_be_born(&cell, &living_neighbours);

    // THEN
    assert_eq!(should_be_born, false);
}
