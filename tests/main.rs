extern crate core;

use std::fs;

#[test]
fn test_only_path() {
    let args: Vec<String> = vec!["csv_to_json", "tests/test.csv"].iter().map(|&x| x.into()).collect();
    let r = csv_to_json::run(args);
    match r {
        Ok(_t) => assert_eq!(true, true),
        _ => assert_eq!(false, true)
    }
}

#[test]
fn test_input_output() {
    let input = "tests/test.csv";
    let output = "tests/test.json";
    let args: Vec<String> = vec!["csv_to_json", "--input", input, "--output", output].iter().map(|&x| x.into()).collect();
    let r = csv_to_json::run(args);
    match r {
        Ok(_t) => {
            let content = fs::read_to_string(output).unwrap();
            assert_eq!(content, "{\"header_1\":\"Value_1\",\"header2\":\"value_2\"}\n");
            fs::remove_file(output).unwrap();
        }
        _ => assert_eq!(false, true)
    }
}

#[test]
fn test_input_by_globbing() {
    let input = "tests/*.csv";
    let args: Vec<String> = vec!["csv_to_json", "--input", input].iter().map(|&x| x.into()).collect();
    let r = csv_to_json::run(args);
    match r {
        Ok(_t) => {
            let content = fs::read_to_string("tests/test.csv.json").unwrap();
            assert_eq!(content, "{\"header_1\":\"Value_1\",\"header2\":\"value_2\"}\n");
            fs::remove_file("tests/test.csv.json").unwrap();
        }
        _ => assert_eq!(false, true)
    }
}

#[test]
fn test_input_output_by_globbing() {
    let input = "tests/*.csv";
    let output = "";
    let args: Vec<String> = vec!["csv_to_json", "--input", input, "--output", output].iter().map(|&x| x.into()).collect();
    let r = csv_to_json::run(args);
    match r {
        Ok(_t) => {
            let content = fs::read_to_string("tests/test.csv.json").unwrap();
            assert_eq!(content, "{\"header_1\":\"Value_1\",\"header2\":\"value_2\"}\n");
            fs::remove_file("tests/test.csv.json").unwrap();
        }
        _ => assert_eq!(false, true)
    }
}

