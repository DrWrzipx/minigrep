use std::process;
use std::env;
use minigrep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem with reading arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}


