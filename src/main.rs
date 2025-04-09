use std::env;
use std::process;

use file_search::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);

    });

    eprintln!("Searching for {}", config.query);
    eprintln!("In file {}", config.file_path);

    if let Err(e) = file_search::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}