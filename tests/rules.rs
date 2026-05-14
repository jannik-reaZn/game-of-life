use game_of_life::domain::rules::{born, survive, die};
use game_of_life::domain::cell::Cell;
use game_of_life::domain::state::State;

#[test]
fn test_survive_rule() {
    let neighbours = vec![
        Cell::new(State::Live, Vec::new()),
        Cell::new(State::Live, Vec::new()),
        Cell::new(State::Live, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
    ];
    let cell = Cell::new(State::Dead, neighbours);

    assert_eq!(cell.neighbours().len(), 8);
    let has_survived = survive(cell);
    assert_eq!(has_survived, true);
}

#[test]
fn test_born_rule() {
    let neighbours = vec![
        Cell::new(State::Live, Vec::new()),
        Cell::new(State::Live, Vec::new()),
        Cell::new(State::Live, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
    ];
    let cell = Cell::new(State::Dead, neighbours);
    assert_eq!(born(cell), true);
}

#[test]
fn test_die_rule() {
    let neighbours = vec![
        Cell::new(State::Live, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
        Cell::new(State::Dead, Vec::new()),
    ];
    let cell = Cell::new(State::Dead, neighbours);
    assert_eq!(die(cell), true);
}
