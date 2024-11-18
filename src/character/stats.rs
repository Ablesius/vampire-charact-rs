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

    /// Create a [Health] struct from a [Character]'s values.
    /// Health in VtM is calculated as 3 + [Stamina](Attribute::Stamina),
    /// so we only need to extract Stamina from the
    /// [Attributes](crate::Attributes) field.
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
    /// Willpower is calculated from a character's [Attribute::Composure] + [Attribute::Resolve]] values
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Humanity {
    pub value: u8,
    pub stains: u8,
}

impl Default for Humanity {
    /// We don't want humanity to default to 0, since that would mean our vampire has fallen to the Beast.
    fn default() -> Self {
        Self {
            value: 7,
            stains: 0,
        }
    }
}

impl Humanity {
    /// The default is to start with humanity 7, but exceptions apply:
    ///
    /// 1. Ancillae: see [Humanity::new_for_ancilla].
    /// 2. by predator type or flaw: These can reduce humanity further. See the rulebooks for details.
    pub fn new(&self) -> Self {
        Self::default()
    }

    /// Ancillae start with their humanity reduced by one.
    pub fn new_for_ancilla(&self) -> Self {
        Self {
            value: 6,
            stains: 0,
        }
    }

    pub fn from_character(character: &Character) -> Self {
        Self {
            value: character.humanity.value,
            stains: character.humanity.stains,
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

        let char = Character::builder()
            .player_name(String::from("Juke"))
            .character_name(String::from("Mx Anderson"))
            .chronicle(String::from("Cthulhu by Night"))
            .attributes(attributes)
            .build();

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

    #[test]
    fn humanity_7_for_neonates() {
        let char = Character::from_file(std::path::PathBuf::from(
            "tests/sample_character_dir/sample_char.json",
        ))
        .expect("sample_char.json should contain valid character json!");

        let expected_humanity = Humanity {
            value: 7,
            stains: 1,
        };

        assert_eq!(Humanity::from_character(&char), expected_humanity);
    }

    #[test]
    fn humanity_6_for_ancillae() {
        // TODO: write ancilla to file, then use from_file
        let char = Character {
            player_name: "".to_string(),
            character_name: "".to_string(),
            chronicle: "".to_string(),
            attributes: Default::default(),
            skills: Default::default(),
            damage: Default::default(),
            willpower_damage: Default::default(),
            humanity: Humanity {
                value: 6,
                stains: 0,
            },
            blood_potency: Default::default(),
            generation: 13.into(),
            hunger: Default::default(),
        };

        let expected_humanity = Humanity {
            value: 6,
            stains: 0,
        };

        assert_eq!(Humanity::from_character(&char), expected_humanity);
    }
}
