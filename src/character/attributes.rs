use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

/// Attributes that a character has.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub strength: u8,
    pub dexterity: u8,
    pub stamina: u8,

    pub charisma: u8,
    pub manipulation: u8,
    pub composure: u8,

    pub intelligence: u8,
    pub wits: u8,
    pub resolve: u8,
}

impl Attributes {
    /// Set all attributes to 2 dots. We can use this with a mut instance
    /// to then later change values individually.
    pub fn set_all_to_2(&mut self) {
        self.strength = 2;
        self.dexterity = 2;
        self.stamina = 2;
        self.charisma = 2;
        self.manipulation = 2;
        self.composure = 2;
        self.intelligence = 2;
        self.wits = 2;
        self.resolve = 2;
    }

    /// Set attributes during character creation.
    /// The rules require that there's one attribute at 4 dots, one at 1, three at 3, and the rest
    /// at 2.
    pub fn set_attributes_during_creation(
        &mut self,
        highest: Attribute,
        lowest: Attribute,
        three_threes: Vec<Attribute>,
    ) {
        self.set_all_to_2();
        self[highest] = 4;
        self[lowest] = 1;

        for attr in three_threes {
            self[attr] = 3;
        }
    }
}

/// We can use this to do something like
/// let attr: Attribute = Attribute::Stamina/* say you get this from user input ... */;
/// character.attributes\[attr] += 1;
impl Index<Attribute> for Attributes {
    type Output = u8;

    fn index(&self, attr: Attribute) -> &Self::Output {
        match attr {
            Attribute::Strength => &self.strength,
            Attribute::Dexterity => &self.dexterity,
            Attribute::Stamina => &self.stamina,
            Attribute::Charisma => &self.charisma,
            Attribute::Manipulation => &self.manipulation,
            Attribute::Composure => &self.composure,
            Attribute::Intelligence => &self.intelligence,
            Attribute::Wits => &self.wits,
            Attribute::Resolve => &self.resolve,
        }
    }
}

// snip: from dmitrii @ Discord
// fn is_attribute_at_least_threshold(&self, attribute: Attribute, threshold: u8) -> bool {
//     self.attributes[attribute] >= threshold
// }

impl IndexMut<Attribute> for Attributes {
    fn index_mut(&mut self, index: Attribute) -> &mut Self::Output {
        match index {
            Attribute::Strength => &mut self.strength,
            Attribute::Dexterity => &mut self.dexterity,
            Attribute::Stamina => &mut self.stamina,
            Attribute::Charisma => &mut self.charisma,
            Attribute::Manipulation => &mut self.manipulation,
            Attribute::Composure => &mut self.composure,
            Attribute::Intelligence => &mut self.intelligence,
            Attribute::Wits => &mut self.wits,
            Attribute::Resolve => &mut self.resolve,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Strength,
    Dexterity,
    Stamina,
    Charisma,
    Manipulation,
    Composure,
    Intelligence,
    Wits,
    Resolve,
}

// /// Build an attribute with a value and its proper category.
// ///
// /// No attribute value can be greater than 5, in accordance with the rules of V5.
// /// See Core Rulebook, p. 155, "Core Traits".
// pub fn build(value: u8, /* category: AttributeCategory */) -> Result<Attribute, anyhow::Error> {
//     if value > 5 {
//         return Err(anyhow!("Attribute cannot be over 5!"));
//     }
//
//     Ok(Attribute {
//         value, /*category*/
//     })
// }

impl FromStr for Attribute {
    type Err = ParseAttributeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "s" | "strength" => Attribute::Strength,
            "d" | "dexterity" => Attribute::Dexterity,
            "t" | "stamina" => Attribute::Stamina,
            "c" | "charisma" => Attribute::Charisma,
            "m" | "manipulation" => Attribute::Manipulation,
            "o" | "composure" => Attribute::Composure,
            "i" | "intelligence" => Attribute::Intelligence,
            "w" | "wits" => Attribute::Wits,
            "r" | "resolve" => Attribute::Resolve,
            _ => return Err(ParseAttributeError),
        })
    }
}

/// An error that occurs when an `Attribute` should be parsed
/// from a `String`, and the `String` has no adequate representation
/// in any `Attribute`.
#[derive(Debug)]
pub struct ParseAttributeError;

impl Display for ParseAttributeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "string did not match any attribute".fmt(f)
    }
}

impl std::error::Error for ParseAttributeError {}

#[test]
fn set_all_to_two_works() {
    let mut test_attributes = Attributes::default();
    let expected = Attributes {
        strength: 2,
        dexterity: 2,
        stamina: 2,
        charisma: 2,
        manipulation: 2,
        composure: 2,
        intelligence: 2,
        wits: 2,
        resolve: 2,
    };

    test_attributes.set_all_to_2();
    assert_eq!(test_attributes, expected);
}
