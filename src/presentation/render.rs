use crate::presentation::board_renderer::{BoardRenderer, TerminalBoardRenderer};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn run() {
    const BOARD_SIZE: usize = 30;
    const MAX_ITERATIONS: usize = 500;
    const FRAME_DELAY_MS: u64 = 120;

    // Initialize board renderer
    let mut board_renderer = TerminalBoardRenderer::new();

    // Seed initial board
    board_renderer.seed(BOARD_SIZE);

    for generation in 0..=MAX_ITERATIONS {
        // Clear screen and move cursor to top-left for a smooth frame-by-frame animation.
        print!("\x1B[2J\x1B[H");
        println!("");
        println!("Generation {generation}/{MAX_ITERATIONS}:");
        println!("{}", board_renderer.render());
        io::stdout()
            .flush()
            .expect("failed to flush terminal output");

        if generation == MAX_ITERATIONS {
            break;
        }

        board_renderer.run_one_generation();
        thread::sleep(Duration::from_millis(FRAME_DELAY_MS));
    }
}
