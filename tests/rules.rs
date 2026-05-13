use game_of_life::domain::rules::{born, survive};

#[test]
fn test_survive_rule() {
    assert_eq!(survive(), true);
}

#[test]
fn test_born_rule() {
    assert_eq!(born(), true);
}