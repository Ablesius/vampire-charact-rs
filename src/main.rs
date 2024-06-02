use clap::Parser;
use std::process;
use vampire_charact_rs::{run, Cli};

fn main() {
    let args = Cli::parse();

    if let Err(e) = run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
