use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

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
