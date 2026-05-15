use game_of_life::application::use_cases::run_one_generation::run_one_generation;
use game_of_life::domain::board::Board;
use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;

#[test]
fn test_blinker_pattern() {
    // Starting pattern:
    // . X .
    // . X .
    // . X .

    // Next generation pattern:
    // . . .
    // X X X
    // . . .

    // GIVEN
    let mut board = Board::new(vec![
        vec![
            Cell::new(State::Dead, Cell::create_neighbours(0)),
            Cell::new(State::Live, Cell::create_neighbours(1)),
            Cell::new(State::Dead, Cell::create_neighbours(0)),
        ],
        vec![
            Cell::new(State::Dead, Cell::create_neighbours(3)),
            Cell::new(State::Live, Cell::create_neighbours(2)),
            Cell::new(State::Dead, Cell::create_neighbours(3)),
        ],
        vec![
            Cell::new(State::Dead, Cell::create_neighbours(0)),
            Cell::new(State::Live, Cell::create_neighbours(1)),
            Cell::new(State::Dead, Cell::create_neighbours(0)),
        ],
    ]);

    // WHEN
    run_one_generation(&mut board);

    // THEN
    assert_eq!(board.cells().len(), 3);
    assert_eq!(board.cells()[0].len(), 3);
    assert_eq!(board.cells()[0][0].state(), State::Dead);
    assert_eq!(board.cells()[0][1].state(), State::Dead);
    assert_eq!(board.cells()[0][2].state(), State::Dead);
    assert_eq!(board.cells()[1][0].state(), State::Live);
    assert_eq!(board.cells()[1][1].state(), State::Live);
    assert_eq!(board.cells()[1][2].state(), State::Live);
    assert_eq!(board.cells()[2][0].state(), State::Dead);
    assert_eq!(board.cells()[2][1].state(), State::Dead);
    assert_eq!(board.cells()[2][2].state(), State::Dead);
}
