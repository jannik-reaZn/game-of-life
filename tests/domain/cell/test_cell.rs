use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;
use rstest::rstest;

#[rstest]
#[case(State::Live)]
#[case(State::Dead)]
fn test_cell_state(#[case] state: State) {
    // GIVEN
    let cell = Cell::new(state);

    // THEN
    assert_eq!(cell.state(), state);
}

#[rstest]
#[case(State::Live, State::Dead)]
#[case(State::Dead, State::Live)]
fn test_set_cell_state(#[case] initial_state: State, #[case] new_state: State) {
    // GIVEN
    let mut cell = Cell::new(initial_state);

    // WHEN
    cell.set_state(new_state);

    // THEN
    assert_eq!(cell.state(), new_state);
}
