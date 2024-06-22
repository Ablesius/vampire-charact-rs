pub mod character;

use crate::character::Character;
use clap::{Parser, Subcommand};
use std::error::Error;
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
    /// Interactively create a new character
    Create {},
}

/// Iterate over a directory and find json files
/// that we use as character "sheets".
pub fn json_paths(dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    // Code originally written by Carnagion#5942 @ Rust Prog Lang Community Discord.
    // Explanations by harudagondi#1480.
    // `fs::read_dir` return a result of `ReadDir`, which is an iterator of `Result<DirEntry, Error>`.
    fs::read_dir(dir)?
        // a `filter_map` is a `filter` and a `map` combined, as indicated in the name.
        // the filtering mechanism is done through returning an `Option`:
        // `Some` if you want to keep the value, `None` to discard it.
        .filter_map(|entry|
                // `entry.map` here means it calls `Result::map`,
                // which is essentially just turning `Result<T, E>` to `Result<U, E>`
                // or in other words, mapping the success value to a different value
                // (with possibly a different type)
                entry.map(|entry| {
                    // inside the `map`, carnagion gets the path of the directory,
                    // then its extension, then returns a boolean whether or not the `extension()`
                    // returned a Some with a value of "json".
                    // `then_some` just turns a boolean into a `Some`
                    // with the value given if it is true, `None` otherwise
            let path = entry.path();
            let is_json = path.extension().is_some_and(|ext| ext == "json");
            is_json.then_some(path)
        // going back, `entry.map` in this context turns a `Result<DirEntry, Error>`
        // into `Result<Option<PathBuf>, Error>`.
        // however, `filter_map` wants an `Option`. `transpose` just turns the result
        // inside out so it becomes `Option<Result<PathBuf, Error>>`
        }).transpose())
        .collect()
}

/// List character files found in a directory.
pub fn list_characters(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let paths: Vec<PathBuf> = json_paths(path)?;
    // next thing we wanna do: go through the files and return
    // Characters from them.
    let characters: Vec<Character> = paths
        .iter()
        .filter_map(|p| match Character::from_file(p) {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Error processing character sheet: {}", e);
                None
            }
        })
        .collect();
    for c in &characters {
        println!(
            "Player: {}, Character: {}",
            c.player_name(),
            c.character_name()
        );
    }
    Ok(())
}

/// Print the most important character details:
///
/// The name of the player, of the character, and of the chronicle.
pub fn print_character(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let character = Character::from_file(path)?;
    println!("Player: {}", character.player_name());
    println!("Character: {}", character.character_name());
    println!("Chronicle: {}", character.chronicle());
    Ok(())
}

pub fn create_character() -> Result<(), Box<dyn Error>> {
    println!("Welcome to character creation!");
    println!("What's your name (for display on the character sheet)?");
    let mut input_player_name = String::new();
    io::stdin().read_line(&mut input_player_name)?;

    println!("Thanks!");
    println!("What's your character's name? (You can provide a first and last name, or multiple names, or just a nickname. Whatever you like!)");
    let mut input_char_name = String::new();
    io::stdin().read_line(&mut input_char_name)?;

    println!("Thanks!");
    println!("What's the name of the chronicle?");
    let mut input_chronicle = String::new();
    io::stdin().read_line(&mut input_chronicle)?;

    let character = Character::new(input_player_name, input_char_name, input_chronicle);
    println!("{:#?}", character);
    Ok(())
}
