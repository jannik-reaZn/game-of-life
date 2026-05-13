use game_of_life::domain::rules::{born, survive, die};

#[test]
fn test_survive_rule() {
    assert_eq!(survive(), true);
}

#[test]
fn test_born_rule() {
    assert_eq!(born(), true);
}

#[test]
fn test_die_rule() {
    assert_eq!(die(), true);
}
