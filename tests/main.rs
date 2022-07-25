extern crate core;

use std::fs;
use csv_to_json::ApplicationOptions;

#[test]
fn test_only_path() {
    let options = ApplicationOptions { input: vec!["test.csv".to_owned()], output: None };

    let r = csv_to_json::run_by_option(&options);
    match r {
        Ok(_t) => assert_eq!(true, true),
        _ => assert_eq!(false, true)
    }
}

#[test]
fn test_input_output() {
    let input = vec!["tests/test.csv".to_string()];
    let output : Option<String> = Some("tests/test.json".to_string());

    let options: ApplicationOptions = ApplicationOptions { input, output};
    let r = csv_to_json::run_by_option(&options);
    match r {        Ok(_t) => {
            let content = fs::read_to_string("tests/test.json").unwrap();
            assert_eq!(content, "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n");
            fs::remove_file("tests/test.json").unwrap();
        }
        _ => assert_eq!(false, true)
    }
}

#[test]
fn test_input_by_globbing() {
    let input = vec!["tests/*.csv".to_string()];
    let output = Some("output".to_string());

    let options: ApplicationOptions = ApplicationOptions { input, output};
    let r = csv_to_json::run_by_option(&options);
    match r {
        Ok(_t) => {
            let content = fs::read_to_string("output/tests/test.csv.json").unwrap();
            assert_eq!(content, "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n");
            fs::remove_dir_all("output").unwrap();
        }
        _ => assert_eq!(false, true)
    }
}

#[test]
fn test_input_output_by_globbing() {
    let input = vec!["tests/*.csv".to_string()];
    let output = None;

    let options: ApplicationOptions = ApplicationOptions { input, output};
    let r = csv_to_json::run_by_option(&options);
    match r {
        Ok(_t) => {
            let content = fs::read_to_string("tests/test.csv.json").unwrap();
            assert_eq!(content, "{\"header_1\":\"Value_1\",\"header_2\":\"value_2\"}\n");
            fs::remove_file("tests/test.csv.json").unwrap();
        }
        _ => assert_eq!(false, true)
    }
}

