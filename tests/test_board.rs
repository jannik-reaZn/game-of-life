use game_of_life::domain::board::Board;
use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;
use rstest::{fixture, rstest};

#[fixture]
fn sample_cells() -> Vec<Vec<Cell>> {
    vec![
        vec![
            Cell::new(State::Live),
            Cell::new(State::Dead),
            Cell::new(State::Dead),
        ],
        vec![
            Cell::new(State::Dead),
            Cell::new(State::Live),
            Cell::new(State::Dead),
        ],
        vec![
            Cell::new(State::Dead),
            Cell::new(State::Dead),
            Cell::new(State::Live),
        ],
    ]
}

#[rstest]
fn test_board_initialization(sample_cells: Vec<Vec<Cell>>) {
    // GIVEN
    let cells = sample_cells.clone();

    // WHEN
    let board = Board::new(cells.clone());

    // THEN
    assert_eq!(board.cells(), &cells);
}

#[rstest]
fn test_board_mutation(sample_cells: Vec<Vec<Cell>>) {
    // GIVEN
    let mut board = Board::new(sample_cells.clone());

    // WHEN
    board.cells_mut()[0][0].set_state(State::Dead);

    // THEN
    assert_eq!(board.cells()[0][0].state(), State::Dead);
}

#[rstest]
fn test_get_living_neighbour(sample_cells: Vec<Vec<Cell>>) {
    // GIVEN
    let board = Board::new(sample_cells.clone());

    // WHEN
    let living_neighbours_cell_0_0 = board.get_living_neighbour(0, 0);
    let living_neighbours_cell_0_1 = board.get_living_neighbour(0, 1);

    // THEN
    assert_eq!(living_neighbours_cell_0_0, 1);
    assert_eq!(living_neighbours_cell_0_1, 2);
}

#[rstest]
fn test_get_neighbour_cells(sample_cells: Vec<Vec<Cell>>) {
    // GIVEN
    let board = Board::new(sample_cells.clone());

    // WHEN
    let neighbour_cells = board.get_neighbour_cells(1, 1);

    // THEN
    let living_neighbours = neighbour_cells
        .iter()
        .filter(|cell| cell.state() == State::Live)
        .count();

    assert_eq!(neighbour_cells.len(), 8);
    assert_eq!(living_neighbours, 2);
}
