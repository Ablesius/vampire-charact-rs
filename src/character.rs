pub mod attributes;
pub mod blood;
pub mod skills;
pub mod stats;

use crate::character::blood::{BloodPotency, Generation, Hunger};
use crate::character::stats::{Damage, Health, Humanity, Willpower};
use anyhow::Result;
pub use attributes::Attribute;
use attributes::Attributes;
use bon::builder;
use bon::Builder;
use serde::{Deserialize, Serialize};
use skills::Skills;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

#[derive(PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
/// Create a new Character with mostly default values.
///
/// There is no constructor; we're using the builder pattern instead.
/// It is assumed that you will use other ways of actually creating a character,
/// like using the `create` command or the GUI. (TODO: not implemented yet)
///
/// You can provide [Attributes] and [Skills] or leave them blank (by explicitly passing [None]);
/// with [None], the default values will be set (0 for attributes and (0, None) for skills;
/// see [Skills].
///
/// **Note**: We assume that a new character does not have any [Damage];
/// that would have to be set later.
pub struct Character {
    pub player_name: String,
    pub character_name: String,
    pub chronicle: String,

    // TODO in the future, neither attributes nor skills should be unset, but for now we can
    //  live with them being all zeros.
    #[builder(default)]
    pub attributes: Attributes,
    #[builder(default)]
    pub skills: Skills,

    // Initialize it with its `Default` value
    #[builder(skip)]
    pub damage: Damage,
    #[builder(skip)]
    pub willpower_damage: Damage,
    #[builder(skip)]
    pub humanity: Humanity,

    // TODO make BP optional in ::new, 1 default;
    #[builder(default)]
    pub blood_potency: BloodPotency,
    // TODO make Gen optional in ::new, 13 default;
    #[builder(default)]
    pub generation: Generation,
    #[builder(default)]
    pub hunger: Hunger,
}

impl Character {
    /// Parse a json file and return [Result<Character>].
    ///
    /// # JSON format
    /// For the Option types, the JSON has to look similar to this:
    /// ```json
    /// "{\"name\":\"Foo\",\"brawl_skill\":[0,null]}"
    ///```
    /// (In other words, `null` represents what [None] is serialized to).
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
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }

    pub fn print(&self) {
        println!("Player: {}", self.player_name);
        println!("Chronicle: {}", self.chronicle);

        println!("Character: {}", self.character_name);

        // TODO: print all the fields? or just most important?
        println!("Attributes: {:?}", self.attributes);
        println!("Skills: {:?}", self.skills);
        println!(
            "Health & Damage: {:?}",
            Health::from_character(
                self,
                Some(self.damage.superficial),
                Some(self.damage.aggravated)
            )
        );
        // println!("Hunger: {:?}", self.hunger);
        // TODO note: when using :? we print the Debug print, which gives `BloodPotency(1)` or similar;
        //  we'll need to impl a Display that only prints the value or something that reads more nicely
        println!("Blood Potency: {:?}", self.blood_potency);
    }

    //TODO do we need this rather?
    fn _get_max_health(&self) -> u8 {
        Health::from_character(
            self,
            Some(self.damage.superficial),
            Some(self.damage.aggravated),
        )
        .value
    }

    // TODO same as above
    fn _get_max_wp(&self) -> u8 {
        Willpower::from_character(self).value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_character_all_default_values() {
        let test_char = Character::builder()
            .player_name(String::from("Test Player"))
            .character_name(String::from("Test Character"))
            .chronicle(String::from("Test Chronicle by Night"))
            .build();

        assert_eq!(
            test_char,
            Character {
                player_name: String::from("Test Player"),
                character_name: String::from("Test Character"),
                chronicle: String::from("Test Chronicle by Night"),
                attributes: Attributes::default(),
                skills: Skills::default(),
                damage: Damage::default(),
                willpower_damage: Damage::default(),
                humanity: Humanity::default(),
                blood_potency: BloodPotency::default(),
                generation: Default::default(),
                hunger: Default::default(),
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
        let test_char = Character::builder()
            .player_name(String::from("Test Player"))
            .character_name(String::from("Test Character"))
            .chronicle(String::from("Test Chronicle by Night"))
            .attributes(attributes)
            .build();

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
            skills: Skills::default(),
            damage: Damage::default(),
            willpower_damage: Damage::default(),
            humanity: Humanity::default(),
            blood_potency: BloodPotency::default(),
            generation: Default::default(),
            hunger: Default::default(),
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

        let test_char = Character::builder()
            .player_name(String::from(""))
            .character_name(String::from(""))
            .chronicle(String::from(""))
            .skills(skills)
            .build();

        let expected = Character {
            player_name: String::from(""),
            character_name: String::from(""),
            chronicle: String::from(""),
            attributes: Attributes::default(),
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
            damage: Default::default(),
            willpower_damage: Default::default(),
            humanity: Default::default(),
            hunger: Default::default(),
            blood_potency: Default::default(),
            generation: Default::default(),
        };

        assert_eq!(test_char, expected);
    }
}
