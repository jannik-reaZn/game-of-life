fn main() {
    if let Err(error) = game_of_life::presentation::render::run() {
        eprintln!("application error: {error}");
    }
}
