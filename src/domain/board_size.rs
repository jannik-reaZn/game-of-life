#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardSize {
    rows: usize,
    cols: usize,
}

impl BoardSize {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn square(size: usize) -> Self {
        Self {
            rows: size,
            cols: size,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}