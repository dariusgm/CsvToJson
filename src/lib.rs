use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{MAIN_SEPARATOR, Path};

use csv::{Reader, StringRecord};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use parsing::ApplicationOptions;

mod parsing;

pub fn convert_data(options: &ApplicationOptions) {
    if !Path::exists(Path::new(&options.input)) {
        panic!("{:?}", &options.input);
    }

    let mut rdr = Reader::from_path(&options.input).unwrap();
    let headers: Vec<String> = rdr.headers()
        .unwrap()
        .iter()
        .map(|s| String::from(s).replace('\"', "\\\""))
        .collect();

    rdr.records().for_each(|optional_record| {
        for record in optional_record {
            let mut line = "{".to_owned();
            headers.iter().enumerate().for_each(|(i, h)| {
                let value = (record.get(i).unwrap()).to_string();
                line.push('"');
                line.push_str(&h);
                line.push_str("\":\"");
                line.push_str(&value.replace('\"', "\\\""));
                line.push_str("\",");
            });


            let mut a = line[0..line.len() - 1].to_string();
            a.push_str("}\n");
            println!("{:?}", a)
        }
    });
}

fn to_absolute(option: &ApplicationOptions, path: &Path) -> String {
    let last = option.input.split(MAIN_SEPARATOR).last().unwrap();
    let last_with_separator = format!("{}{}", String::from(MAIN_SEPARATOR), String::from(last));
    let prefix = option.input.replace(&last_with_separator, &String::from(""));
    format!(
        "{}/{}.{}",
        prefix,
        path.file_stem().unwrap().to_str().unwrap(),
        path.extension().unwrap().to_str().unwrap(),
    )
}
/*
fn run_files_channel(options: &ApplicationOptions) {
    let mut files_to_process = Vec::new();

    // Prepare data for processing
    for entry in glob::glob(&options.input).unwrap() {
        match entry {
            Ok(path) => {
                let file_name = path.display();
                println!("{:?}", file_name);

                let mut patched_options = options.clone();
                patched_options.input = to_absolute(&patched_options, &path);

                match &options.output {
                    None => patched_options.output = Some(format!("{}.json", file_name)),
                    Some(x) => patched_options.output = Some(format!("{}/{}.json", x, file_name))
                }

                files_to_process.push(patched_options)
            }

            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    }

    files_to_process.par_iter().for_each(process)
}
*/
pub fn run_by_str(args: Vec<&str>) -> Result<(), Box<dyn Error>> {
    run(args.iter().map(|&x| x.into()).collect())
}

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{:?}", args);
    let options = parsing::arg_parse();
        match &options.output {
            _ => { convert_data(&options);}
        }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::assert_eq;
    use std::collections::HashMap;

    use csv::StringRecord;

    #[test]
    fn test_build_json_line() {
        use super::build_json_line;
        let mut data = HashMap::new();
        let mut header = StringRecord::new();
        header.push_field("test-key");
        data.insert(String::from("test-key"), String::from("test-value"));
        let json = build_json_line(data, header);
        assert_eq!(json, "{\"test-key\":\"test-value\"}\n")
    }

    #[test]
    fn test_build_json_line_with_double_quotes() {
        use super::build_json_line;
        let mut data = HashMap::new();
        let mut header = StringRecord::new();
        header.push_field("test-key");
        data.insert(String::from("test-key"), String::from("test\"-value"));
        let json = build_json_line(data, header);
        assert_eq!(json, "{\"test-key\":\"test\\\"-value\"}\n")
    }
}
