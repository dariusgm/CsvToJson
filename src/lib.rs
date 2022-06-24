use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{MAIN_SEPARATOR, Path};

use csv::{Reader, StringRecord};
use log::{error, Record};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use parsing::ApplicationOptions;

mod parsing;


pub fn convert_line(headers: &Vec<String>, record: &StringRecord) -> String {
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
    a
}

pub fn file_handler(output: &Option<String>) -> bool {
    match output {
        Some(x) => true,
        None => false
    }
}

pub fn write_to_file(mut rdr: Reader<File>, headers: &Vec<String>, output: String) {
    let mut file_handler = File::create(output).unwrap();
    rdr.records().for_each(|optional_record| {
        for record in optional_record {
            let converted_line_output = convert_line(&headers, &record);
            let _ = file_handler.write_all(&converted_line_output.as_bytes());
        }
    });
}

pub fn write_to_stdout(mut rdr: Reader<File>, headers: &Vec<String>) {
    rdr.records().for_each(|optional_record| {
        for record in optional_record {
            let converted_line_output = convert_line(&headers, &record);
            println!("{}", converted_line_output);
        }
    });
}
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

    let out_file = file_handler(&options.output.clone());
    if out_file {
        write_to_file(rdr, &headers, options.output.clone().unwrap())
    } else {
        write_to_stdout(rdr, &headers)
    }
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
