use std::env;
use std::process;
use vampire_charact_rs::Config;

/// Vampire-Charact-rs
///
/// A Vampire: The Masquerade Character Manager written in Rust
///
/// Usage:
/// `vampire-charact-rs FOLDER`
/// will list characters that are found in that folder.
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = vampire_charact_rs::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
