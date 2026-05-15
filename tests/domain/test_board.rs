use game_of_life::domain::board::Board;
use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;

#[test]
fn test_board_initialization() {
    // GIVEN
    let cells = vec![
        vec![Cell::new(State::Live), Cell::new(State::Dead)],
        vec![Cell::new(State::Dead), Cell::new(State::Live)],
    ];

    // WHEN
    let board = Board::new(cells.clone());

    // THEN
    assert_eq!(board.cells(), &cells);
}
