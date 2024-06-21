use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(PartialEq, Debug, Default, Deserialize)]
pub struct Character {
    player_name: String,
    character_name: String,
}

impl Character {
    pub fn new(player_name: String, character_name: String) -> Character {
        Character {
            player_name,
            character_name,
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
