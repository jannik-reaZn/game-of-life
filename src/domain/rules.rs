use super::cell::Cell;
use super::state::State;

/// A live cell with two or three live neighbours survives.
pub fn survive(cell: Cell) -> bool {
    let neighbours = cell.neighbours();
    let mut nof_life_neighbours= 0 as i8;
    for neighbour in neighbours {
       match neighbour.state() {
        State::Live => nof_life_neighbours += 1,
        State::Dead => continue,
       }
    }
    if nof_life_neighbours == 2 || nof_life_neighbours == 3 {
        return true
    }
    false
}

/// A dead cell with exactly three live neighbours becoomes a live cell.
pub fn born(cell: Cell) -> bool {
    let neighbours = cell.neighbours();
    let mut nof_life_neighbours= 0 as i8;
    for neighbour in neighbours {
       match neighbour.state() {
        State::Live => nof_life_neighbours += 1,
        State::Dead => continue,
       }
    }
    if nof_life_neighbours == 3 {
        return true
    }
    false
}

/// A live cell with fewer than two or more than three live neighbours die
pub fn die(cell: Cell) -> bool {
    let neighbours = cell.neighbours();
    let mut nof_life_neighbours= 0 as i8;
    for neighbour in neighbours {
       match neighbour.state() {
        State::Live => nof_life_neighbours += 1,
        State::Dead => continue,
       }
    }

    if nof_life_neighbours < 2 || nof_life_neighbours > 3 {
        return true
    }
    false
}