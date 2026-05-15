use crate::presentation::board_renderer::{BoardRenderer, TerminalBoardRenderer};

pub fn run() {
    // Initialize board renderer
    let mut board_renderer = TerminalBoardRenderer::new();

    // Seed and render initial board
    board_renderer.seed();
    println!("Initial board:");
    println!("{}", board_renderer.render());

    // Run one generation and render again
    board_renderer.run_one_generation();
    println!("\nAfter one generation:");
    println!("{}", board_renderer.render());
}
