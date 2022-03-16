use std::env;

use laydown::parse_arguments;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    parse_arguments(arguments);
}
