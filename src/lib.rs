use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
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

#[derive(PartialEq, Debug, Default, Deserialize)]
pub struct Character {
    player_name: String,
    character_name: String,
}

pub fn list_characters(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let paths: Vec<PathBuf> = json_paths(path)?;
    // next thing we wanna do: go through the files and return
    // Characters from them.
    let characters: Vec<Character> = paths
        .iter()
        .filter_map(|p| match character_from_file(p) {
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

impl Character {
    pub fn new(player_name: String, character_name: String) -> Character {
        Character {
            player_name,
            character_name,
        }
    }

    pub fn player_name(&self) -> &String {
        &self.player_name
    }
    pub fn character_name(&self) -> &String {
        &self.character_name
    }
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

/// Parse a json file and return a Result<Character, boxed Error>.
pub fn character_from_file<P: AsRef<Path>>(path: P) -> Result<Character, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let character = serde_json::from_reader(reader)?;

    Ok(character)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_char() {
        let test_char = Character::new(String::from("Test Player"), String::from("Test Character"));
        assert_eq!(
            test_char,
            Character {
                player_name: String::from("Test Player"),
                character_name: String::from("Test Character"),
            }
        );
    }

    #[test]
    fn char_name() {
        // Test that the character_name method behaves as expected.
        let test_char = Character::new(String::from("Test Player"), String::from("Test Character"));

        let expected_char_name = String::from("Test Character");
        assert_eq!(test_char.character_name, expected_char_name);
    }

    #[test]
    fn player_name() {
        // Test that the player_name method behaves as expected.
        let test_char = Character::new(String::from("Test Player"), String::from("Test Character"));

        let expected_player_name = String::from("Test Player");
        assert_eq!(test_char.player_name, expected_player_name);
    }
}
