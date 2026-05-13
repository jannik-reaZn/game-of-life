/// A live cell with two or three live neighbours survives.
pub fn survive() -> bool {
    true
}

/// A dead cell with exactly three live neighbours becoomes a live cell.
pub fn born() -> bool {
    true
}

/// A live cell with fewer than two or more than three live neighbours die
pub fn die() -> bool {
    true
}