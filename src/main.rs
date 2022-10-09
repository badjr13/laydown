#![warn(clippy::pedantic)]
fn main() {
    if let Err(e) = laydown::get_args().and_then(laydown::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
