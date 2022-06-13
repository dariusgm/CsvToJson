use std::env;

/** This converts a csv file to a json file.
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    csv_to_json::run(args).unwrap()
}
