use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(PartialEq, Debug, Default, Deserialize)]
pub struct Character {
    player_name: String,
    character_name: String,
    chronicle: String,
}

impl Character {
    pub fn new(player_name: String, character_name: String, chronicle: String) -> Character {
        Character {
            player_name,
            character_name,
            chronicle,
        }
    }

    /// Parse a json file and return a Result<Character, boxed Error>.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Character, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let character = serde_json::from_reader(reader)?;

        Ok(character)
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
    fn new_char() {
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
            }
        );
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
