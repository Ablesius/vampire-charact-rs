use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Debug, Default, Deserialize, Serialize)]
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

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
pub struct BloodPotency(u8);

impl Default for BloodPotency {
    /// Default BloodPotency should be one, not 0, because only Thin-Blood vampires have 0
    /// (which is possible to be and have, but it is not the default case).
    fn default() -> Self {
        Self(1)
    }
}

impl From<u8> for BloodPotency {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl BloodPotency {
    /// Automatically assign [BloodPotency] based on the provided [Generation].
    /// This is not intended as an absolute conversion table, as it is possible to buy
    /// higher values in [BloodPotency] with XP, and it is possible to change [Generation]
    /// e.g. through diablerie.
    /// In order to keep things simple, any generation lower than 10 just gets their [BloodPotency] set to 3.
    /// Adjust yourself where necessary.
    pub(crate) fn from_generation(generation: &Generation) -> Self {
        match generation.0 {
            ..10 => 3,
            10..=11 => 2,
            12..=13 => 1,
            14.. => 0,
        }
        .into()
    }
}

/// Generation, a value between 1 and 16 (although higher values *are* allowed; it is possible
/// that you're playing a chronicle with extremely high-generation Thin-Bloods, for example).
///
/// In order to check that Generation is always > 0, use `let gen: Generation = 0.into()` and
/// don't construct with `Generation(0)`; the latter **will** have 0 as value, not 1.
/// TODO make Generation private, would that help avoiding the 0 case?
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
pub struct Generation(u8);

impl Default for Generation {
    /// Default Generation is 13, since that makes for a good starting point with new [Characters](crate::Character).
    fn default() -> Self {
        13.into()
    }
}

impl From<u8> for Generation {
    /// Get the generation from just a number above 0.
    ///
    /// Theoretically it makes sense to restrict extremely low generations,
    /// but this is then a problem of the character *creation* process.
    /// Nothing stops us from having character sheets for methuselahs.
    ///
    /// The only restriction is that generation cannot be 0; in that case it will be set to 1 instead.
    fn from(value: u8) -> Self {
        if value == 0 {
            eprintln!("Generation cannot be 0, setting to 1 instead!");
            return Self(1);
        }
        Self(value)
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
        let generation: Generation = 10.into();
        let bp = BloodPotency::from_generation(&generation);

        // for 10th generation, BP should be at least 2
        let expected = BloodPotency(2);

        assert!(bp >= expected)
    }

    #[test]
    fn generation_0_turns_1() {
        let gen_0: Generation = 0.into();
        assert_eq!(gen_0, Generation(1));
    }

    #[test]
    fn generation_0_stays_0_when_using_normal_construction() {
        //! see docstring of [Generation] above!
        let gen_0 = Generation(0);
        assert_eq!(gen_0, Generation(0));
    }
}
