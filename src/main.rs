use clap::Parser;
use std::process;
use vampire_charact_rs::{list_characters, print_character};
use vampire_charact_rs::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { path } => {
            let p = path.unwrap_or_else(|| {
                std::env::current_dir().expect("should be able to access its own directory")
            });
            if let Err(e) = list_characters(p) {
                println!("Application error: {e}");
                process::exit(1);
            }
        }
        Commands::Print { path } => {
            if let Some(p) = path {
                if let Err(e) = print_character(p) {
                    println!("Could not print character: {e}");
                    process::exit(1);
                }
            } else {
                println!("Application error: You need to provide a file for `print`!");
                process::exit(1);
            }
        }
        Commands::Add { path: _ } => {
            println!("not implemented yet!");
            process::exit(2);
        }
    }
}
