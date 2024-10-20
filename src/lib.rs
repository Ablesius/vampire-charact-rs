pub mod character;

use crate::character::attributes::Attributes;
use crate::character::Character;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io, path::Path};

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
                eprintln!("Error processing character sheet {}: {}", p.display(), e);
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
/// The name of the player, the character, and the chronicle.
pub fn print_character(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let character = Character::from_file(path)?;
    character.print();
    Ok(())
}

/// Create a character by interactively providing the fields it requires.
pub fn create_character() -> Result<()> {
    let attribute_selection_prompt: &str = "
    [S]trength, [D]exterity, S[t]amina,
    [C]harisma, [M]anipulation, C[o]mposure,
    [I]ntelligence, [W]its or [R]esolve";

    println!("Welcome to character creation!");
    let input_player_name =
        read_user_input("What's your name (for display on the character sheet)?")?;

    println!("Thanks!");
    let input_char_name = read_user_input(
        "What's your character's name? (You can provide a first and last name, or multiple names, or just a nickname. Whatever you like!)"
    )?;

    println!("Thanks!");
    let input_chronicle = read_user_input("What's the name of the chronicle?")?;

    println!("Thanks!");

    println!("Now we need to distribute your attributes.");

    println!("Select one attribute to assign 4 dots to, by typing the whole name or just the highlighted letter:");
    let highest = read_user_input(attribute_selection_prompt)?.parse::<character::Attribute>()?;
    println!("{:?} selected for 4 dots.", highest);
    println!();

    println!("Now select the attribute to only assign 1 dot to:");
    let lowest = read_user_input(attribute_selection_prompt)?.parse::<character::Attribute>()?;
    println!("{:?} selected for 1 dot.", lowest);
    println!();

    println!("Now select three attributes with 3 dots each.");
    println!("First 3-dot attribute:");
    let _3_dots_1 = read_user_input(attribute_selection_prompt)?.parse::<character::Attribute>()?;
    println!("{:?} selected for 3 dots.", _3_dots_1);
    println!("Second 3-dot attribute:");
    let _3_dots_2 = read_user_input(attribute_selection_prompt)?.parse::<character::Attribute>()?;
    println!("{:?} selected for 3 dots.", _3_dots_2);
    println!("Third 3-dot attribute:");
    let _3_dots_3 = read_user_input(attribute_selection_prompt)?.parse::<character::Attribute>()?;
    println!("{:?} selected for 3 dots.", _3_dots_3);

    // map the highest, lowest, 3-dot and 2-dot attributes to the attribute enum
    let mut attributes: Attributes = Default::default();
    attributes.set_all_to_2();
    attributes[highest] = 4;
    attributes[lowest] = 1;
    attributes[_3_dots_1] = 3;
    attributes[_3_dots_2] = 3;
    attributes[_3_dots_3] = 3;

    println!();
    let character = Character::new(
        input_player_name,
        input_char_name,
        input_chronicle,
        Some(attributes),
        None,
    );
    // TODO: this block is just for debugging purposes, remove later
    println!("{:#?}", character);

    println!();
    let file_path = read_user_input("Where would you like to save your character? You may provide an absolute path or one relative to this directory. Please provide a filename with the '.json' ending, e.g. './character.json' or '/some/other/path/my_characters_name.json'.")?;
    character
        .to_file(file_path)
        .context("Could not write character to file")?;
    Ok(())
}

/// Prompt the user to input something and return it as io::Result<String>.
fn read_user_input(instruction: &str) -> io::Result<String> {
    print!("{instruction}: ");
    // This version in particular has both the prompt and the user's answer
    // on the same line, which looks a bit nicer than
    // ```
    // please input here:
    // > place for the user to input
    // ```
    io::stdout().flush()?;
    let mut buff = String::new();
    io::stdin().read_line(&mut buff)?;
    Ok(buff.trim().to_owned())
}

/// Iterate over a directory and find json files
/// that we use as character sheets.
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
