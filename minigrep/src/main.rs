use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not enough args passed!");
        return;
    }

    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // println!("Searching for: {}", config.query);
    // println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
