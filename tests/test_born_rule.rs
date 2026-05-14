use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::born;
use game_of_life::domain::state::State;

#[test]
fn test_born_rule() {
    let cell = Cell::new(State::Dead, Cell::create_neighbours(3));
    assert_eq!(born(&cell), true);
}
