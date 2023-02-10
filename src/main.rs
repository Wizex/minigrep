use std::{env, process};
use minigrep::Config;

fn main() {
    let cfg = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {err}. Exiting...");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(cfg) {
        eprintln!("Error: {err}. Exiting...");
        process::exit(1);
    }
}


