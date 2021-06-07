use std::process;
use std::env;
use minigrep_tutorial_matic::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem with reading arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = minigrep_tutorial_matic::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}


