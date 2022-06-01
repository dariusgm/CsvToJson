use std::collections::HashMap;
use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn arg_parse(args: Vec<String>) -> Options {
    let input: String = String::from("--input");
    let output: String = String::from("--output");
    let quiet: String = String::from("--quiet");

    let mut options = Options::default();
    for (i, a) in args.iter().enumerate() {
        if input.eq(a) {
            let input_csv = args[i + 1_usize].clone();
            options.input = input_csv;
        }

        if output.eq(a) {
            let output_json = args[i + 1_usize].clone();
            options.output = output_json;
        }

        if quiet.eq(a) {
            options.quiet = true;
        }
    }
    return options;
}

pub fn build_json_line(record: HashMap<String, String>) -> String {
    let mut line = "{".to_string();
    for (k, v) in record {
        line.push('"');
        line.push_str(&k.replace("\"", "\\\""));
        line.push_str("\":\"");
        line.push_str(&v.replace("\"", "\\\""));
        line.push_str("\",");
    }
    // remove last comma
    let mut a = line[0..line.len() - 1].to_string();
    a.push_str("}\n");
    a
}

pub fn read_data(options: &Options) -> Vec<HashMap<String, String>> {
    let mut rdr = Reader::from_path(&options.input).unwrap();
    let headers = rdr.headers().unwrap().clone();
    let data: Vec<HashMap<String, String>> = rdr
        .records()
        .map(|record| {
            Ok(headers
                .iter()
                .map(|e| e.to_string())
                .zip(record?.iter().map(String::from))
                .collect())
        })
        .collect::<Result<_, Box<dyn Error>>>().unwrap();
    data
}

pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let data = read_data(&options);
    if options.output.is_empty() {
        for record in data {
            let line = build_json_line(record);
            print!("{}", line)
        }
    } else {
        let mut file_handler = File::create(&options.output).unwrap();
        for record in data {
            let line = build_json_line(record);
            let b = line.as_bytes();
            file_handler.write_all(b).unwrap();
        }
    }
    Ok(())
}

pub struct Options {
    pub input: String,
    pub output: String,
    pub quiet: bool,
}

impl Clone for Options {
    fn clone(&self) -> Self {
        Options {
            input: self.input.clone(),
            output: self.output.clone(),
            quiet: self.quiet.clone(),
        }
    }
}


impl Default for Options {
    fn default() -> Self {
        Self {
            input: String::from("*.csv"),
            output: String::from("."),
            quiet: false,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn test_build_json_line() {
        use super::build_json_line;
        let mut data = HashMap::new();
        data.insert(String::from("test-key"), String::from("test-value"));
        let json = build_json_line(data);
        assert_eq!(json, "{\"test-key\":\"test-value\"}\n")
    }

    #[test]
    fn test_buid_json_line_with_doublequotes() {
        use super::build_json_line;
        let mut data = HashMap::new();
        data.insert(String::from("test-key"), String::from("test\"-value"));
        let json = build_json_line(data);
        assert_eq!(json, "{\"test-key\":\"test\\\"-value\"}\n")
    }
}
