use std::env;

use laydown::parse_arguments;
use laydown::Env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    parse_arguments(arguments, Env::Prod);
}
