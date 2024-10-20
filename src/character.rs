pub mod attributes;
pub mod skills;

use anyhow::Result;
pub use attributes::Attribute;
use attributes::Attributes;
use serde::{Deserialize, Serialize};
use skills::Skills;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

#[derive(PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Character {
    player_name: String,
    character_name: String,
    chronicle: String,

    attributes: Attributes,
    skills: Skills,
}

impl Character {
    /// Create a new Character.
    /// You can provide attributes and skills or leave them blank (by explicitly passing `None`);
    /// with `None`, the default values will be set (0 for attributes and (0, None) for skills;
    /// see `Skills`.
    pub fn new(
        player_name: String,
        character_name: String,
        chronicle: String,
        attributes: Option<Attributes>,
        skills: Option<Skills>,
    ) -> Self {
        Self {
            player_name,
            character_name,
            chronicle,
            attributes: attributes.unwrap_or_default(),
            skills: skills.unwrap_or_default(),
        }
    }

    /// Parse a json file and return `anyhow::Result<Character>`.
    ///
    /// # JSON format
    /// For the Option types, the JSON has to look similar to this:
    /// ```json
    /// "{\"name\":\"Foo\",\"brawl_skill\":[0,null]}"
    ///```
    /// (In other words, `null` represents what is `Option::None` as serialized.
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
        // TODO: print all the fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn attributes_and_skills_unset() {
        let test_char = Character::new(
            String::from("Test Player"),
            String::from("Test Character"),
            String::from("Test Chronicle by Night"),
            None,
            None,
        );

        assert_eq!(
            test_char,
            Character {
                player_name: String::from("Test Player"),
                character_name: String::from("Test Character"),
                chronicle: String::from("Test Chronicle by Night"),
                attributes: Attributes::default(),
                skills: Default::default(),
            }
        );
    }

    #[test]
    fn attributes_passed_explicitly() {
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
        let test_char = Character::new(
            String::from("Test Player"),
            String::from("Test Character"),
            String::from("Test Chronicle by Night"),
            Some(attributes),
            None,
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
            skills: Default::default(),
        };

        assert_eq!(test_char, expected);
    }

    #[test]
    fn skills_passed_explicitly_no_specialities() {
        let skills = Skills {
            athletics: (1, None),
            brawl: (2, None),
            craft: (3, None),
            drive: (4, None),
            firearms: (5, None),
            larceny: (0, None),
            melee: (1, None),
            stealth: (2, None),
            survival: (3, None),
            animal_ken: (4, None),
            etiquette: (5, None),
            insight: (0, None),
            intimidation: (1, None),
            leadership: (2, None),
            performance: (3, None),
            persuasion: (4, None),
            streetwise: (5, None),
            subterfuge: (0, None),
            academics: (1, None),
            awareness: (2, None),
            finance: (3, None),
            investigation: (4, None),
            medicine: (5, None),
            occult: (0, None),
            politics: (1, None),
            science: (2, None),
            technology: (3, None),
        };

        let test_char = Character::new(
            String::from(""),
            String::from(""),
            String::from(""),
            Some(Attributes::default()),
            Some(skills),
        );

        let expected = Character {
            player_name: String::from(""),
            character_name: String::from(""),
            chronicle: String::from(""),
            attributes: Default::default(),
            skills: Skills {
                athletics: (1, None),
                brawl: (2, None),
                craft: (3, None),
                drive: (4, None),
                firearms: (5, None),
                larceny: (0, None),
                melee: (1, None),
                stealth: (2, None),
                survival: (3, None),
                animal_ken: (4, None),
                etiquette: (5, None),
                insight: (0, None),
                intimidation: (1, None),
                leadership: (2, None),
                performance: (3, None),
                persuasion: (4, None),
                streetwise: (5, None),
                subterfuge: (0, None),
                academics: (1, None),
                awareness: (2, None),
                finance: (3, None),
                investigation: (4, None),
                medicine: (5, None),
                occult: (0, None),
                politics: (1, None),
                science: (2, None),
                technology: (3, None),
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
            None,
            None,
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
            None,
            None,
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
            None,
            None,
        );

        let expected = String::from("Test Chronicle");
        assert_eq!(test_char.chronicle, expected);
    }
}
