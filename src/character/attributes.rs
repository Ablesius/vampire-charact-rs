use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::Index;
use std::str::FromStr;

/// Attributes that a character has.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    str: u8,
    dex: u8,
    sta: u8,

    cha: u8,
    man: u8,
    com: u8,

    int: u8,
    wit: u8,
    res: u8,
}

impl Attributes {}

/// We can use this to do something like
/// let attr: Attribute = Attribute::Stamina/* say you get this from user input ... */;
/// character.attributes\[attr] += 1;
impl Index<Attribute> for Attributes {
    type Output = u8;

    fn index(&self, attr: Attribute) -> &Self::Output {
        match attr {
            Attribute::Strength => &self.str,
            Attribute::Dexterity => &self.dex,
            Attribute::Stamina => &self.sta,
            Attribute::Charisma => &self.cha,
            Attribute::Manipulation => &self.man,
            Attribute::Composure => &self.com,
            Attribute::Intelligence => &self.int,
            Attribute::Wits => &self.wit,
            Attribute::Resolve => &self.res,
        }
    }
}

// snip: from dmitrii @ Discord
// fn is_attribute_at_least_threshold(&self, attribute: Attribute, threshold: u8) -> bool {
//     self.attributes[attribute] >= threshold
// }

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
