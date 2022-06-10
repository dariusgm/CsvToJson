use std::env;
use csv_to_json::arg_parse;

/** This converts a csv file to a json file.
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    let options = arg_parse(args);
    csv_to_json::run(options).unwrap()
}
