use std::path::PathBuf;
use vampire_charact_rs::character::attributes::Attributes;
use vampire_charact_rs::character::skills::Skills;
use vampire_charact_rs::character::stats::{Damage, Humanity};
use vampire_charact_rs::character::{Attribute, Character};
use vampire_charact_rs::*;

#[test]
fn sample_char_dir_three_results() {
    let dir = "tests/sample_character_dir";
    let expected_jsons = [
        "tests/sample_character_dir/sample_char.json",
        "tests/sample_character_dir/sample_char_2.json",
        "tests/sample_character_dir/sample_char_3.json",
    ]
    .map(PathBuf::from);

    let mut results = json_paths(&dir).expect("test dir should contain json files");
    results.sort();

    assert_eq!(results, expected_jsons);
}

#[test]
fn no_results_in_dir() {
    let dir = "tests/empty_char_dir";
    let result = json_paths(&dir);
    let empty_vec: Vec<PathBuf> = vec![];
    assert_eq!(result.expect("this should be an empty folder"), empty_vec);
}

#[test]
fn new_char_from_sample_json() {
    let expected = Character {
        player_name: String::from("Jason"),
        character_name: String::from("Phil Rubens"),
        chronicle: String::from("Something by Night"),
        attributes: Attributes {
            strength: 1,
            dexterity: 3,
            stamina: 2,
            charisma: 4,
            manipulation: 3,
            composure: 2,
            intelligence: 2,
            wits: 3,
            resolve: 3,
        },
        skills: Skills {
            athletics: (0, None),
            brawl: (0, None),
            craft: (3, Some(String::from("plastic arts"))),
            drive: (0, None),
            firearms: (0, None),
            larceny: (0, None),
            melee: (0, None),
            stealth: (1, None),
            survival: (0, None),
            animal_ken: (0, None),
            etiquette: (3, None),
            insight: (2, None),
            intimidation: (1, None),
            leadership: (2, None),
            performance: (3, Some(String::from("public speaking"))),
            persuasion: (3, Some(String::from("seduction"))),
            streetwise: (0, None),
            subterfuge: (2, None),
            academics: (2, Some(String::from("art history"))),
            awareness: (1, None),
            finance: (2, Some(String::from("stock market"))),
            investigation: (0, None),
            medicine: (0, None),
            occult: (1, None),
            politics: (1, None),
            science: (0, None),
            technology: (0, None),
        },
        damage: Damage::default(),
        willpower_damage: Damage::default(),
        humanity: Humanity {
            value: 7,
            stains: 1,
        },
        blood_potency: 1.into(),
        generation: 13.into(),
        hunger: 1.into(),
    };

    let char = Character::from_file(PathBuf::from("tests/sample_character_dir/sample_char.json"))
        .expect("sample_char.json should contain valid character json!");

    assert_eq!(char, expected)
}

#[test]
fn new_char_from_sample_2() {
    let expected = Character {
        player_name: String::from("Mary"),
        character_name: String::from("Cassandra Skyloft"),
        chronicle: String::from("Let the Streets Run Red"),
        attributes: Attributes {
            strength: 1,
            dexterity: 3,
            stamina: 2,
            charisma: 4,
            manipulation: 3,
            composure: 2,
            intelligence: 2,
            wits: 3,
            resolve: 2,
        },
        skills: Skills::default(),
        damage: Default::default(),
        willpower_damage: Default::default(),
        humanity: Default::default(),
        hunger: 0.into(),
        blood_potency: Default::default(),
        generation: 12.into(),
    };

    let char = Character::from_file(PathBuf::from(
        "tests/sample_character_dir/sample_char_2.json",
    ))
    .expect("sample_char_2.json should contain valid character json!");

    assert_eq!(char, expected)
}

#[test]
fn new_char_from_sample_3() {
    let expected = Character {
        player_name: String::from("Jib"),
        character_name: String::from("Mordred"),
        chronicle: String::from("Something by Night"),

        attributes: Attributes::default(),
        skills: Skills::default(),

        damage: Damage {
            superficial: 3,
            aggravated: 1,
        },
        willpower_damage: Damage {
            superficial: 2,
            aggravated: 0,
        },

        humanity: Humanity {
            value: 7,
            stains: 1,
        },
        blood_potency: 2.into(),
        generation: 14.into(),
        hunger: 3.into(),
    };

    let char = Character::from_file(PathBuf::from(
        "tests/sample_character_dir/sample_char_3.json",
    ))
    .expect("sample_char_3.json should contain valid character json!");

    assert_eq!(char, expected)
}

#[test]
#[should_panic]
fn faulty_char_produces_error() {
    Character::from_file(PathBuf::from("tests/faulty_char_sheet/faulty_char.json"))
        .expect("This character sheet does not contain all the required fields!");
}

#[test]
fn new_char_to_file() {
    let mut attributes = Attributes::default();
    let highest = Attribute::Dexterity;
    let lowest = Attribute::Resolve;
    let three_mid = vec![
        Attribute::Charisma,
        Attribute::Intelligence,
        Attribute::Manipulation,
    ];

    attributes.set_attributes_during_creation(highest, lowest, three_mid);

    let skills = Skills {
        athletics: (1, None),
        brawl: (2, None),
        craft: (3, Some(String::from("carpenter"))),
        drive: (0, None),
        firearms: (0, None),
        larceny: (0, None),
        melee: (1, None),
        stealth: (0, None),
        survival: (1, Some(String::from("foraging"))),
        animal_ken: (2, None),
        etiquette: (0, None),
        insight: (1, None),
        intimidation: (2, None),
        leadership: (1, Some(String::from("practicality"))),
        performance: (0, None),
        persuasion: (1, None),
        streetwise: (1, None),
        subterfuge: (0, None),
        academics: (0, None),
        awareness: (3, None),
        finance: (0, None),
        investigation: (2, None),
        medicine: (1, None),
        occult: (0, None),
        politics: (0, None),
        science: (0, None),
        technology: (0, None),
    };

    let bp = 2;

    let char = Character::new(
        String::from("Test player"),
        String::from("Test character"),
        String::from("Test chronicle"),
        Some(attributes),
        Some(skills),
        None,
        Some(bp.into()),
        None,
    );

    char.to_file(PathBuf::from(
        "tests/integration_test_characters/test_character.json",
    ))
    .expect("couldn't write to test output file!");
}
