use crate::presentation::board_renderer::{BoardRenderer, TerminalBoardRenderer};

pub fn run() {
    // Initialize board renderer
    let mut board_renderer = TerminalBoardRenderer::new();

    // Seed the board and render it
    board_renderer.seed();

    // Render the board and print the output
    let output = board_renderer.render();
    println!("{}", output);
}
