use std::path::PathBuf;
use vampire_charact_rs::character::attributes::Attributes;
use vampire_charact_rs::character::Character;
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
    let expected = Character::new_with_attributes(
        String::from("Jason"),
        String::from("Phil Rubens"),
        String::from("Something by Night"),
        Attributes {
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
    );
    let char = Character::from_file(PathBuf::from("tests/sample_character_dir/sample_char.json"))
        .expect("sample_char.json should contain valid character json!");

    assert_eq!(char, expected)
}

#[test]
fn new_char_from_sample_2() {
    let expected = Character::new_with_attributes(
        String::from("Mary"),
        String::from("Cassandra Skyloft"),
        String::from("Let the Streets Run Red"),
        Attributes {
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
    );
    let char = Character::from_file(PathBuf::from(
        "tests/sample_character_dir/sample_char_2.json",
    ))
    .expect("sample_char_2.json should contain valid character json!");

    assert_eq!(char, expected)
}

#[test]
fn new_char_from_sample_3() {
    let expected = Character::new_with_attributes(
        String::from("Jib"),
        String::from("Mordred"),
        String::from("Something by Night"),
        Attributes {
            strength: 0,
            dexterity: 0,
            stamina: 0,
            charisma: 0,
            manipulation: 0,
            composure: 0,
            intelligence: 0,
            wits: 0,
            resolve: 0,
        },
    );
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
