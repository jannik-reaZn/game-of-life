use game_of_life::application::use_cases::handle_living_cell_use_case::handle_living_cell;
use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;
use rstest::rstest;

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
#[case(4)]
#[case(8)]
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

#[test]
fn test_handle_living_cell_when_cell_is_dead_does_not_change_state() {
    // GIVEN
    let mut cell = Cell::new(State::Dead, Cell::create_neighbours(1));

    // WHEN
    handle_living_cell(&mut cell);

    // THEN
    assert_eq!(cell.state(), State::Dead);
}
