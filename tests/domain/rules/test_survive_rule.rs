use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::should_survive;
use game_of_life::domain::state::State;
use rstest::rstest;

#[rstest]
#[case(2)]
#[case(3)]
fn test_living_cell_has_survived(#[case] living_neighbours: usize) {
    // GIVEN
    let cell = Cell::new(State::Live);

    // WHEN
    let has_survived = should_survive(&cell, &living_neighbours);

    // THEN
    assert_eq!(has_survived, true);
}

#[rstest]
#[case(1)]
#[case(4)]
fn test_living_cell_has_not_survived(#[case] living_neighbours: usize) {
    // GIVEN
    let cell = Cell::new(State::Live);

    // WHEN
    let has_survived = should_survive(&cell, &living_neighbours);

    // THEN
    assert_eq!(has_survived, false);
}

#[test]
fn test_dead_cell_unchanged() {
    // GIVEN
    let cell = Cell::new(State::Dead);
    let living_neighbours = 1;

    // WHEN
    let has_survived = should_survive(&cell, &living_neighbours);

    // THEN
    assert_eq!(has_survived, false);
}
