use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::Index;
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Skills {
    pub athletics: (u8, Option<String>),
    pub brawl: (u8, Option<String>),
    pub craft: (u8, Option<String>),
    pub drive: (u8, Option<String>),
    pub firearms: (u8, Option<String>),
    pub larceny: (u8, Option<String>),
    pub melee: (u8, Option<String>),
    pub stealth: (u8, Option<String>),
    pub survival: (u8, Option<String>),
    pub animal_ken: (u8, Option<String>),
    pub etiquette: (u8, Option<String>),
    pub insight: (u8, Option<String>),
    pub intimidation: (u8, Option<String>),
    pub leadership: (u8, Option<String>),
    pub performance: (u8, Option<String>),
    pub persuasion: (u8, Option<String>),
    pub streetwise: (u8, Option<String>),
    pub subterfuge: (u8, Option<String>),
    pub academics: (u8, Option<String>),
    pub awareness: (u8, Option<String>),
    pub finance: (u8, Option<String>),
    pub investigation: (u8, Option<String>),
    pub medicine: (u8, Option<String>),
    pub occult: (u8, Option<String>),
    pub politics: (u8, Option<String>),
    pub science: (u8, Option<String>),
    pub technology: (u8, Option<String>),
}

// impl Default for Skills {
//     fn default() -> Self {
//         Self {
//             athletics: (0, None),
//             brawl: (0, None),
//             craft: (0, None),
//             drive: (0, None),
//             firearms: (0, None),
//             larceny: (0, None),
//             melee: (0, None),
//             stealth: (0, None),
//             survival: (0, None),
//             animal_ken: (0, None),
//             etiquette: (0, None),
//             insight: (0, None),
//             intimidation: (0, None),
//             leadership: (0, None),
//             performance: (0, None),
//             persuasion: (0, None),
//             streetwise: (0, None),
//             subterfuge: (0, None),
//             academics: (0, None),
//             awareness: (0, None),
//             finance: (0, None),
//             investigation: (0, None),
//             medicine: (0, None),
//             occult: (0, None),
//             politics: (0, None),
//             science: (0, None),
//             technology: (0, None),
//         }
//     }
// }

/// We can use this to do something like
/// let skill: Skill = Skill::Brawl/* say you get this from user input ... */;
/// character.skills\[brawl] += 1;
impl Index<Skill> for Skills {
    type Output = (u8, Option<String>);

    fn index(&self, skill: Skill) -> &Self::Output {
        match skill {
            Skill::Athletics => &self.athletics,
            Skill::Brawl => &self.brawl,
            Skill::Craft => &self.craft,
            Skill::Drive => &self.drive,
            Skill::Firearms => &self.firearms,
            Skill::Larceny => &self.larceny,
            Skill::Melee => &self.melee,
            Skill::Stealth => &self.stealth,
            Skill::Survival => &self.survival,
            Skill::AnimalKen => &self.animal_ken,
            Skill::Etiquette => &self.etiquette,
            Skill::Insight => &self.insight,
            Skill::Intimidation => &self.intimidation,
            Skill::Leadership => &self.leadership,
            Skill::Performance => &self.performance,
            Skill::Persuasion => &self.persuasion,
            Skill::Streetwise => &self.streetwise,
            Skill::Subterfuge => &self.subterfuge,
            Skill::Academics => &self.academics,
            Skill::Awareness => &self.awareness,
            Skill::Finance => &self.finance,
            Skill::Investigation => &self.investigation,
            Skill::Medicine => &self.medicine,
            Skill::Occult => &self.occult,
            Skill::Politics => &self.politics,
            Skill::Science => &self.science,
            Skill::Technology => &self.technology,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Skill {
    Athletics,
    Brawl,
    Craft,
    Drive,
    Firearms,
    Larceny,
    Melee,
    Stealth,
    Survival,
    AnimalKen,
    Etiquette,
    Insight,
    Intimidation,
    Leadership,
    Performance,
    Persuasion,
    Streetwise,
    Subterfuge,
    Academics,
    Awareness,
    Finance,
    Investigation,
    Medicine,
    Occult,
    Politics,
    Science,
    Technology,
}

impl FromStr for Skill {
    type Err = ParseSkillError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "athletics" => Skill::Athletics,
            "brawl" => Skill::Brawl,
            "craft" => Skill::Craft,
            "drive" => Skill::Drive,
            "firearms" => Skill::Firearms,
            "larceny" => Skill::Larceny,
            "melee" => Skill::Melee,
            "stealth" => Skill::Stealth,
            "survival" => Skill::Survival,
            "animal ken" => Skill::AnimalKen,
            "etiquette" => Skill::Etiquette,
            "insight" => Skill::Insight,
            "intimidation" => Skill::Intimidation,
            "leadership" => Skill::Leadership,
            "performance" => Skill::Performance,
            "persuasion" => Skill::Persuasion,
            "streetwise" => Skill::Streetwise,
            "subterfuge" => Skill::Subterfuge,
            "academics" => Skill::Academics,
            "awareness" => Skill::Awareness,
            "finance" => Skill::Finance,
            "investigation" => Skill::Investigation,
            "medicine" => Skill::Medicine,
            "occult" => Skill::Occult,
            "politics" => Skill::Politics,
            "science" => Skill::Science,
            "technology" => Skill::Technology,
            _ => return Err(ParseSkillError),
        })
    }
}

/// An error that occurs when a `Skill` should be parsed
/// from a `String`, and the `String` has no adequate representation
/// in any `Skill`.
#[derive(Debug)]
pub struct ParseSkillError;

impl Display for ParseSkillError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "string did not match any attribute".fmt(f)
    }
}

impl std::error::Error for ParseSkillError {}

#[test]
fn skill_from_string() {
    let skill = "athletics".parse::<Skill>().unwrap();
    let expected = Skill::Athletics;

    assert_eq!(skill, expected)
}

#[test]
#[should_panic]
fn non_existing_skill_from_string() {
    let _ = "foo".parse::<Skill>().unwrap();
}
