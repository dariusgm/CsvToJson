use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use csv::{Reader, StringRecord};
use csv_to_json::arg_parse;

/** This converts a csv file to a json file.
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    let options = arg_parse(args);
    csv_to_json::run(options).unwrap()
}
