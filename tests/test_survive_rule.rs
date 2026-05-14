use game_of_life::domain::cell::Cell;
use game_of_life::domain::rules::survive;
use game_of_life::domain::state::State;

#[test]
fn test_survive_rule() {
    let cell = Cell::new(State::Dead, Cell::create_neighbours(3));

    assert_eq!(cell.neighbours().len(), 8);
    let has_survived = survive(&cell);
    assert_eq!(has_survived, true);
}
