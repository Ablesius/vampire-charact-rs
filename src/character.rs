pub mod attributes;

use crate::character::attributes::Attributes;
use anyhow::Result;
pub use attributes::Attribute;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

#[derive(PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Character {
    player_name: String,
    character_name: String,
    chronicle: String,

    attributes: Attributes,
}

impl Character {
    /// Create a new instance of Character without attribute values.
    /// This will just use 0 as default for every attribute.
    pub fn new(player_name: String, character_name: String, chronicle: String) -> Self {
        Self {
            player_name,
            character_name,
            chronicle,
            attributes: Default::default(),
        }
    }

    /// Create a new instance of Character with all attributes explicitly.
    pub fn new_with_attributes(
        player_name: String,
        character_name: String,
        chronicle: String,
        attributes: Attributes,
    ) -> Self {
        Self {
            player_name,
            character_name,
            chronicle,
            attributes,
        }
    }

    /// Parse a json file and return `anyhow::Result<Character>`.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Character> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let character = serde_json::from_reader(reader)?;

        Ok(character)
    }

    /// Write a character to a json file.
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }

    pub fn player_name(&self) -> &String {
        &self.player_name
    }
    pub fn character_name(&self) -> &String {
        &self.character_name
    }

    pub fn chronicle(&self) -> &String {
        &self.chronicle
    }

    pub fn print(&self) {
        println!("Player: {}", self.player_name);
        println!("Character: {}", self.character_name);
        println!("Chronicle: {}", self.chronicle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_char_attributes_not_set() {
        let test_char = Character::new(
            String::from("Test Player"),
            String::from("Test Character"),
            String::from("Test Chronicle by Night"),
        );
        assert_eq!(
            test_char,
            Character {
                player_name: String::from("Test Player"),
                character_name: String::from("Test Character"),
                chronicle: String::from("Test Chronicle by Night"),
                attributes: Attributes::default(),
            }
        );
    }

    #[test]
    fn new_char_attributes_passed_explicitly() {
        let attributes = Attributes {
            strength: 5,
            dexterity: 4,
            stamina: 3,
            charisma: 2,
            manipulation: 2,
            composure: 2,
            intelligence: 1,
            wits: 3,
            resolve: 2,
        };
        let test_char = Character::new_with_attributes(
            String::from("Test Player"),
            String::from("Test Character"),
            String::from("Test Chronicle by Night"),
            attributes,
        );

        let expected = Character {
            player_name: String::from("Test Player"),
            character_name: String::from("Test Character"),
            chronicle: String::from("Test Chronicle by Night"),
            attributes: Attributes {
                strength: 5,
                dexterity: 4,
                stamina: 3,
                charisma: 2,
                manipulation: 2,
                composure: 2,
                intelligence: 1,
                wits: 3,
                resolve: 2,
            },
        };

        assert_eq!(test_char, expected);
    }

    #[test]
    fn char_name() {
        let test_char = Character::new(
            String::from("_"),
            String::from("Test Character"),
            String::from("_"),
        );

        let expected = String::from("Test Character");
        assert_eq!(test_char.character_name, expected);
    }

    #[test]
    fn player_name() {
        let test_char = Character::new(
            String::from("Test Player"),
            String::from("_"),
            String::from("_"),
        );

        let expected = String::from("Test Player");
        assert_eq!(test_char.player_name, expected);
    }

    #[test]
    fn chronicle() {
        let test_char = Character::new(
            String::from("_"),
            String::from("_"),
            String::from("Test Chronicle"),
        );

        let expected = String::from("Test Chronicle");
        assert_eq!(test_char.chronicle, expected);
    }
}
