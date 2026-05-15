use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::should_die;
use game_of_life::domain::state::State;
use rstest::rstest;

#[rstest]
#[case(1)]
#[case(4)]
fn test_living_cell_dies(#[case] living_neighbours: usize) {
    // GIVEN
    let cell = Cell::new(State::Live);

    // WHEN
    let has_died = should_die(&cell, &living_neighbours);

    // THEN
    assert_eq!(has_died, true);
}

#[rstest]
#[case(2)]
#[case(3)]
fn test_living_cell_stays_alive(#[case] living_neighbours: usize) {
    // GIVEN
    let cell = Cell::new(State::Live);

    // WHEN
    let has_died = should_die(&cell, &living_neighbours);

    // THEN
    assert_eq!(has_died, false);
}

#[test]
fn test_dead_cell_stays_dead() {
    // GIVEN
    let cell = Cell::new(State::Dead);
    let living_neighbours = 0;

    // WHEN
    let has_died = should_die(&cell, &living_neighbours);

    // THEN
    assert_eq!(has_died, false);
}