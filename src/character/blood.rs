use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Hunger(u8);

impl From<u8> for Hunger {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl PartialEq<u8> for Hunger {
    fn eq(&self, other: &u8) -> bool {
        &self.0 == other
    }
}

impl Hunger {
    /// Create a new instance of Hunger, with a guarantee that it will not exceed 5. If the call is made with a value >5, the value is just set to 5 instead. Otherwise, the provided value will be used.
    ///
    /// # Arguments
    ///
    /// * `value`: The value to set Hunger to. If a number >5 is provided, it will be force-set to 5 instead.
    ///
    /// returns: Hunger
    ///
    /// # Examples
    ///
    /// ```rust
    /// use self::vampire_charact_rs::character::blood::Hunger;
    /// let hunger = Hunger::new(3);
    ///
    /// let hunger_over_5 = Hunger::new(6);
    ///
    /// assert_eq!(hunger, 3);
    /// assert_eq!(hunger_over_5, 5);
    /// ```
    pub fn new(value: u8) -> Self {
        if value > 5u8 {
            return Self(5);
        }
        Self(value)
    }

    pub fn is_in_range(&self) -> bool {
        // we don't need to assert that it's 0 or more,
        // since unsigned integers are always non-negative
        self.0 <= 5u8
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct BloodPotency(u8);

impl From<u8> for BloodPotency {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl PartialOrd for BloodPotency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.cmp(&other.0).into()
    }
}

impl BloodPotency {
    //TODO refactor so that it's used
    pub(crate) fn from_generation(generation: &u8) -> Self {
        match generation {
            10..=11 => 2,
            12..=13 => 1,
            14.. => 0,
            //TODO refactor so that we won't be able to reach panic
            _ => panic!("invalid generation selected, choose between 9 and 16"),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hunger_between_0_and_5() {
        let hunger = Hunger(3);

        assert!(hunger.is_in_range());

        let hunger2 = Hunger(6);
        assert!(!hunger2.is_in_range())
    }

    #[test]
    fn u8_into_blood_potency() {
        let bp: BloodPotency = 3.into();
        let expected = BloodPotency(3);

        assert_eq!(bp, expected)
    }

    #[test]
    fn blood_potency_from_u8() {
        let bp = BloodPotency::from(3);
        let expected = BloodPotency(3);

        assert_eq!(bp, expected)
    }

    #[test]
    fn blood_potency_from_generation() {
        let generation = 10;
        let bp = BloodPotency::from_generation(&generation);

        // for 10th generation, BP should be at least 2
        let expected = BloodPotency(2);

        assert!(bp >= expected)
    }
}
