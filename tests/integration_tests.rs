use std::path::PathBuf;
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
    assert_eq!(result.unwrap(), empty_vec);
}

#[test]
fn new_char_from_sample_json() {
    let expected_char = Character::new(String::from("Jason"), String::from("Phil Rubens"));
    let char = Character::from_file(PathBuf::from("tests/sample_character_dir/sample_char.json"))
        .expect("sample_char.json should contain valid character json!");
    assert_eq!(char, expected_char)
}

#[test]
#[should_panic]
fn faulty_char_produces_error() {
    Character::from_file(PathBuf::from("tests/faulty_char_sheet/faulty_char.json"))
        .expect("This character sheet does not contain all the required fields!");
}
