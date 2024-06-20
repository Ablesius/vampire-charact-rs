pub mod character;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::{fs, io, path::Path};

// https://docs.rs/clap/4.0.18/clap/_derive/_tutorial/index.html#subcommands
//

/// Vampire-Charact-rs
///
/// A Vampire: The Masquerade Character Manager written in Rust.
#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List player and character name per file in the directory
    List { path: Option<PathBuf> },
    /// Print details of a character in a JSON file
    Print { path: Option<PathBuf> },
    /// Add character (noop)
    // TODO: add functionality, then remove "noop"
    Add { path: Option<PathBuf> },
}

/// Iterate over a directory and find json files
/// that we use as character "sheets".
pub fn json_paths(dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    // Code originally written by Carnagion#5942 @ Rust Prog Lang Community Discord.
    // Explanations by harudagondi#1480.
    // `fs::read_dir` return a result of `ReadDir`, which is an iterator of `Result<DirEntry, Error>`.
    fs::read_dir(dir)?
        // a `filter_map` is a `filter` and a `map` combined, as indicated in the name. the filtering mechanism is done through returning an `Option`: `Some` if you want to keep the value, `None` to discard it.
        .filter_map(|entry|
                // `entry.map` here means it calls `Result::map`, which is essentially just turning `Result<T, E>` to `Result<U, E>` or in other words, mapping the success value to a different value (with possibly a different type)
                entry.map(|entry| {
                    // inside the `map`, carnagion gets the path of the directory, then its extension, then returns a boolean whether or not the `extension()` returned a Some with a value of "json". `then_some` just turns a boolean into a `Some` with the value given if it is true, `None` otherwise
            let path = entry.path();
            let is_json = path.extension().is_some_and(|ext| ext == "json");
            is_json.then_some(path)
        // going back, `entry.map` in this context turns a `Result<DirEntry, Error>` into `Result<Option<PathBuf>, Error>`. however, `filter_map` wants an `Option`. `transpose` just turns the result inside out so it becomes `Option<Result<PathBuf, Error>>`
        }).transpose())
        .collect()
}
