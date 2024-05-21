use std::path::PathBuf;
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
