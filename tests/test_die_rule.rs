use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::die;
use game_of_life::domain::state::State;

#[test]
fn test_die_rule() {
    let cell = Cell::new(State::Dead, Cell::create_neighbours(1));
    assert_eq!(die(&cell), true);
}
