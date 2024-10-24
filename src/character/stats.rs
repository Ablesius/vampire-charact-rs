use crate::character::{Attribute, Character};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq)]
pub struct Health {
    pub value: u8,
    pub damage: Damage,
}

impl Health {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            damage: Damage::default(),
        }
    }

    /// Create a Health struct from a Character's values.
    /// Health in VtM is calculated as 3 + Stamina,
    /// so we only need to extract Stamina from the
    /// Attributes field.
    pub fn from_character(
        character: &Character,
        superficial: Option<u8>,
        aggravated: Option<u8>,
    ) -> Self {
        let damage = Damage {
            superficial: superficial.unwrap_or_default(),
            aggravated: aggravated.unwrap_or_default(),
        };

        Self {
            value: &character.attributes[Attribute::Stamina] + 3,
            damage,
        }
    }
}

/// Rather `Damage` than health since we will always be able to calculate max
/// health at runtime. Superficial and aggravated damage, however, must be
/// tracked on the sheet.
#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Damage {
    pub superficial: u8,
    pub aggravated: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Willpower {
    pub value: u8,
    pub damage: Damage,
}

impl Willpower {
    /// Willpower is calculated from a character's Composure + Resolve values
    pub fn from_character(character: &Character) -> Self {
        Self {
            value: character.attributes[Attribute::Composure]
                + character.attributes[Attribute::Resolve],
            damage: Damage {
                superficial: character.willpower_damage.superficial,
                aggravated: character.willpower_damage.aggravated,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_from_character() {
        let char = Character::from_file(std::path::PathBuf::from(
            "tests/sample_character_dir/sample_char.json",
        ))
        .expect("sample_char.json should contain valid character json!");

        let expected_health = Health::new(5);

        assert_eq!(Health::from_character(&char, None, None), expected_health)
    }

    #[test]
    fn health_calculated_correctly() {
        use crate::character::attributes::Attributes;
        let mut attributes = Attributes::default();
        attributes[Attribute::Stamina] = 3;

        let char = Character::new(
            String::from("Juke"),
            String::from("Mx Anderson"),
            String::from("Cthulhu by Night"),
            Some(attributes),
            None,
        );

        let health = Health::from_character(&char, None, None);
        // health should be Stamina + 3, so in this case 6
        let expected = 6;

        assert_eq!(health.value, expected);
    }

    #[test]
    fn willpower_from_character() {
        let char = Character::from_file(std::path::PathBuf::from(
            "tests/sample_character_dir/sample_char.json",
        ))
        .expect("sample_char.json should contain valid character json!");

        let expected_wp = Willpower {
            value: 5,
            damage: Damage::default(),
        };

        assert_eq!(Willpower::from_character(&char), expected_wp);
    }
}
