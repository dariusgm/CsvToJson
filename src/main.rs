use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use csv::Reader;
use std::env;

struct Options {
    input: String,
    output: String,
}

impl Options {
    fn new() -> Self {
        Options {
            input: "".to_string(),
            output: "".to_string(),
        }
    }
}

/** This converts a csv file to a json file.
 */
fn main() {
    let input_csv: String = String::from("--input");
    let output_json: String = String::from("--output");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage:");
        println!("{} \"<path>\"", input_csv);
        println!("{} \"<path>\"", output_json);
        return;
    }
    let mut options = Options::new();
    for (i, a) in args.iter().enumerate() {
        if input_csv.eq(a) {
            let input_csv = args[i + 1_usize].clone();
            options.input = input_csv
        }

        if output_json.eq(a) {
            let output_json = args[i + 1_usize].clone();
            options.output = output_json
        }
    }
    run(options).unwrap()
}

fn build_json_line(record: HashMap<&str, String>) -> String {
    let mut line = "{".to_string();
    for (k, v) in record {
        line.push('"');
        line.push_str(k);
        line.push_str("\":\"");
        line.push_str(v.as_str());
        line.push_str("\",");
    }
    // remove last comma
    let mut a = line[0..line.len() - 1].to_string();
    a.push_str("}\n");
    a
}

fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(options.input)?;
    let headers = rdr.headers()?.clone();

    let data: Vec<HashMap<&str, String>> = rdr
        .records()
        .map(|record| {
            Ok(headers
                .iter()
                .zip(record?.iter().map(String::from))
                .collect())
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

    if options.output.is_empty() {
        for record in data {
            let line = build_json_line(record);
            print!("{}", line)
        }
    } else {
        let mut file_handler = File::create(options.output).unwrap();
        for record in data {
            let line = build_json_line(record);
            let b = line.as_bytes();
            file_handler.write_all(b).unwrap();
        }
    }
    Ok(())
}
