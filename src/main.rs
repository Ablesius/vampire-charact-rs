use clap::Parser;
use std::process;
use vampire_charact_rs::{list_characters, Cli, Commands};

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
        Commands::Print { path: _ } => {
            println!("not implemented yet!");
            process::exit(2);
        }
        Commands::Add { path: _ } => {
            println!("not implemented yet!");
            process::exit(2);
        }
    }
}
