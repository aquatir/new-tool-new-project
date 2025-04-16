use std::{env, process};
use chapter_12_minigrep::{Config, run};


/// A simple search program. Run with
/// ```
/// cargo run -- searchstring poem.txt
/// ```
/// Set env variable `IGNORE_CASE` to any value for case-insensitive search
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
