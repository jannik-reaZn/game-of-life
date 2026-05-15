use crate::application::use_cases::run_one_generation::run_one_generation as advance_board;
use crate::domain::board::Board;
use crate::domain::cell::Cell;
use crate::domain::state::State;
use crate::presentation::cell_renderer::{CellStateRenderer, Renderable};

// Interface for rendering the board
pub trait BoardRenderer {
    fn seed(&mut self);
    fn run_one_generation(&mut self);
    fn render(&self) -> String;
}

// Implementation of BoardRenderer for terminal output
pub struct TerminalBoardRenderer {
    board: Board,
}

impl Default for TerminalBoardRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalBoardRenderer {
    pub fn new() -> Self {
        TerminalBoardRenderer {
            board: Board::new(vec![]),
        }
    }
}

// Implement the BoardRenderer trait for TerminalBoardRenderer
impl BoardRenderer for TerminalBoardRenderer {
    fn seed(&mut self) {
        self.board = Board::new(vec![
            vec![
                Cell::new(State::Dead),
                Cell::new(State::Live),
                Cell::new(State::Dead),
            ],
            vec![
                Cell::new(State::Dead),
                Cell::new(State::Live),
                Cell::new(State::Dead),
            ],
            vec![
                Cell::new(State::Dead),
                Cell::new(State::Live),
                Cell::new(State::Dead),
            ],
        ]);
    }

    fn run_one_generation(&mut self) {
        advance_board(&mut self.board);
    }

    fn render(&self) -> String {
        self.board
            .cells()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        let renderer = CellStateRenderer::new(cell.state());
                        renderer.render()
                    })
                    .map(|c| format!("{} ", c))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
