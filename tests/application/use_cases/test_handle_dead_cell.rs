use game_of_life::application::use_cases::handle_dead_cell_use_case::handle_dead_cell;
use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;
use rstest::rstest;

#[test]
fn test_handle_dead_cell_revives_when_exactly_three_neighbours_are_live() {
    // GIVEN
    let mut cell = Cell::new(State::Dead, Cell::create_neighbours(3));

    // WHEN
    handle_dead_cell(&mut cell);

    // THEN
    assert_eq!(cell.state(), State::Live);
}

#[rstest]
#[case(2)]
#[case(4)]
fn test_handle_dead_cell_stays_dead_when_live_neighbour_count_is_not_three(
    #[case] living_cells: usize,
) {
    // GIVEN
    let mut cell = Cell::new(State::Dead, Cell::create_neighbours(living_cells));

    // WHEN
    handle_dead_cell(&mut cell);

    // THEN
    assert_eq!(cell.state(), State::Dead);
}

#[test]
fn test_handle_dead_cell_keeps_live_cell_unchanged() {
    // GIVEN
    let mut cell = Cell::new(State::Live, Cell::create_neighbours(1));

    // WHEN
    handle_dead_cell(&mut cell);

    // THEN
    assert_eq!(cell.state(), State::Live);
}
