extern crate core;

use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn before(input: &PathBuf) {
    fs::copy(PathBuf::from("tests/test.csv"), input).unwrap();
}

fn after(input: &PathBuf, output: &PathBuf) {
    fs::remove_file(input).unwrap();
    fs::remove_file(output).unwrap()
}

fn cmd() -> Command {
    Command::cargo_bin("csv_to_json").unwrap()
}

#[test]
fn test_show_help() {
    cmd().arg("--help").assert().success().stdout(
        predicate::str::contains("--help")
            .and(predicate::str::contains("--input"))
            .and(predicate::str::contains("--output")),
    );
}

#[test]
fn test_multiple_input() {
    let input_file_for_test_1 = PathBuf::from("tests/test_multi_1.csv");
    let input_file_for_test_2 = PathBuf::from("tests/test_multi_2.csv");
    let output_file_for_test_1 = PathBuf::from("tests/test_multi_1.csv.json");
    let output_file_for_test_2 = PathBuf::from("tests/test_multi_2.csv.json");

    before(&input_file_for_test_1);
    before(&input_file_for_test_2);

    cmd()
        .arg("--input")
        .arg(&input_file_for_test_1)
        .arg(&input_file_for_test_2)
        .assert()
        .success()
        .code(0);

    after(&input_file_for_test_1, &output_file_for_test_1);
    after(&input_file_for_test_2, &output_file_for_test_2);
}

#[test]
fn test_only_path() {
    let input_file_for_test = PathBuf::from("tests/test_only_path.csv");
    let output_file_for_test = PathBuf::from("tests/test_only_path.csv.json");

    before(&input_file_for_test);

    cmd()
        .arg("--input")
        .arg(&input_file_for_test)
        .assert()
        .success()
        .code(0);

    after(&input_file_for_test, &output_file_for_test);
}

#[test]
fn test_input_output() {
    let input_file_for_test = PathBuf::from("tests/test_input_output.csv");
    let output_file_for_test = PathBuf::from("tests/test_input_output_explicit.csv.json");

    before(&input_file_for_test);

    cmd()
        .arg("--input")
        .arg(&input_file_for_test)
        .arg("--output")
        .arg(&output_file_for_test)
        .assert()
        .success()
        .code(0);

    let content = fs::read_to_string(&output_file_for_test).unwrap();
    assert_eq!(
        content,
        "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n"
    );

    after(&input_file_for_test, &output_file_for_test);
}

#[test]
fn test_input_output_by_globbing() {
    fs::create_dir_all(PathBuf::from("tests/input_output_by_globbing")).unwrap();

    let input_file_for_test =
        PathBuf::from("tests/input_output_by_globbing/test_input_by_globbing.csv");
    let output_file_for_test =
        PathBuf::from("output/tests/input_output_by_globbing/test_input_by_globbing.csv.json");

    before(&input_file_for_test);

    cmd()
        .arg("--input")
        .arg("tests/input_output_by_globbing/*.csv")
        .arg("--output")
        .arg("output")
        .assert()
        .success()
        .code(0);

    let content = fs::read_to_string(output_file_for_test).unwrap();
    assert_eq!(
        content,
        "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n"
    );
    fs::remove_dir_all("output").unwrap();
    fs::remove_dir_all("tests/input_output_by_globbing").unwrap();
}

#[test]
fn test_input_by_globbing() {
    fs::create_dir_all(PathBuf::from("tests/input_by_globbing")).unwrap();

    let input_file_for_test = PathBuf::from("tests/input_by_globbing/test_input_by_globbing.csv");
    let output_file_for_test =
        PathBuf::from("tests/input_by_globbing/test_input_by_globbing.csv.json");

    before(&input_file_for_test);

    cmd()
        .arg("--input")
        .arg("tests/input_by_globbing/*.csv")
        .assert()
        .success()
        .code(0);

    let content = fs::read_to_string(output_file_for_test).unwrap();
    assert_eq!(
        content,
        "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n"
    );
    fs::remove_dir_all("tests/input_by_globbing").unwrap();
}
