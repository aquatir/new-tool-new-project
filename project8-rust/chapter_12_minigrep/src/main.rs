use std::{env, process};
use chapter_12_minigrep::{Config, run};

// run with 'cargo run -- searchstring poem.txt'
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
