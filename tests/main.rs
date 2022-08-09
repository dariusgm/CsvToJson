extern crate core;

use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_show_help() {
    let mut command = Command::cargo_bin("csv_to_json").unwrap();
    let assert = command.arg("--help").assert();

    assert.success().stdout(
        predicate::str::contains("--help")
            .and(predicate::str::contains("--input")
    ).and(predicate::str::contains("--output"))
    );
}


#[test]
fn test_only_path() {
    let mut command = Command::cargo_bin("csv_to_json").unwrap();
    let assert = command.arg("--input").arg("tests/test.csv").assert();
    assert.success().code(0);
    fs::remove_file(PathBuf::from("tests/test.csv.json")).unwrap()
}

#[test]
fn test_input_output() {
    let mut command = Command::cargo_bin("csv_to_json").unwrap();
    let assert = command
        .arg("--input")
        .arg("tests/test.csv")
        .arg("--output")
        .arg("tests/test.json")
        .assert();

    assert.success().code(0);

    let content = fs::read_to_string("tests/test.json").unwrap();
    assert_eq!(content, "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n");
    fs::remove_file("tests/test.json").unwrap();

}

#[test]
fn test_input_by_globbing() {

    let mut command = Command::cargo_bin("csv_to_json").unwrap();
    let assert = command
        .arg("--input")
        .arg("tests/*.csv")
        .arg("--output")
        .arg("output")
        .assert();

    assert.success().code(0);

    let content = fs::read_to_string("output/tests/test.csv.json").unwrap();
    assert_eq!(content, "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n");
    fs::remove_dir_all("output").unwrap();

}

#[test]
fn test_input_output_by_globbing() {

    let mut command = Command::cargo_bin("csv_to_json").unwrap();
    let assert = command
        .arg("--input")
        .arg("tests/*.csv")
        .assert();

    assert.success().code(0);

    let content = fs::read_to_string("tests/test.csv.json").unwrap();
    assert_eq!(content, "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n");
    fs::remove_file("tests/test.csv.json").unwrap();
}

