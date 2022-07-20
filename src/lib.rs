use std::borrow::Borrow;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{MAIN_SEPARATOR, Path};

use csv::{Reader, StringRecord};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use clap::Parser;
use crate::parsing::arg_parse;

mod parsing;

#[derive(Parser)]
#[clap(name = "CsvToJson")]
#[clap(version = "0.1")]
#[clap(about = "Converts csv files to json", long_about = None)]
pub struct ApplicationOptions {
    #[clap(long, multiple_values = true)]
    pub input: Vec<String>,

    #[clap(long, value_parser)]
    pub output: Option<String>,
}

#[derive(Debug)]
pub struct ProcessingUnit {
    input: String,
    output: String
}


pub fn convert_line(headers: &[String], record: &StringRecord) -> String {
    let mut line = "{".to_owned();
    headers.iter().enumerate().for_each(|(i, h)| {
        let value = (record.get(i).unwrap()).to_string();
        line.push('"');
        line.push_str(h);
        line.push_str("\":\"");
        line.push_str(&value.replace('\"', "\\\""));
        line.push_str("\",");
    });


    let mut a = line[0..line.len() - 1].to_string();
    a.push_str("}\n");
    a
}

pub fn write_to_file(mut rdr: Reader<File>, headers: &[String], output: &String) {
    if let Ok(mut file_handler) = File::create(output) {
        rdr.records().for_each(|optional_record| {
            if let Ok(record) = optional_record {
                let converted_line_output = convert_line(headers, &record);
                let _ = file_handler.write_all(converted_line_output.as_bytes());
            }
        });
    }
}

pub fn write_to_stdout(mut rdr: Reader<File>, headers: &[String]) {
    rdr.records().for_each(|optional_record| {
        if let Ok(record) = optional_record {
            let converted_line_output = convert_line(headers, &record);
            println!("{}", converted_line_output);
        }
    });
}

fn build_output_path(output: &Option<String>, input: &str, argument: &str, file_name: &OsStr) -> String {
    let file_name_str  = file_name.to_str().unwrap();
    match output {
        None => format!("{}.json", file_name_str),
        Some(x) => {
            match argument.contains('*') {
                // handle as directory for output
                true => {
                    let path_str = format!("{}{}{}", x, MAIN_SEPARATOR, file_name_str);
                    let last = input.split(MAIN_SEPARATOR).last().unwrap();
                    let prefix = path_str.replace(last, "");
                    // create required directory structure in case of globbing
                    let full_output_path = Path::new(&prefix);
                    fs::create_dir_all(full_output_path).unwrap();
                    format!("{}.json", path_str)
                },
                // handle as file
                false => {
                    x.to_string()
                }
            }
        }
    }
}

pub fn collect_files(options: &ApplicationOptions) -> Vec<ProcessingUnit> {
    let mut files_to_process = Vec::new();

    for argument in &options.input {
        for entry in glob::glob(argument).unwrap() {
            match entry {
                Ok(path) => {
                    if let Some(file_name) = path.file_name() {
                        let input = to_absolute(file_name, &path);
                        let output = build_output_path(
                            options.output.borrow(),
                            &input,
                            &argument,
                            file_name,
                        );


                        let processing_unit = ProcessingUnit {
                            input,
                            output
                        };

                        files_to_process.push(processing_unit)
                    }
                }

                // if the path matched but was unreadable,
                // thereby preventing its contents from matching
                Err(e) => println!("{:?}", e),
            }
        }
    }
    files_to_process
}

pub fn convert_data(processing_unit: &ProcessingUnit) {
    if !Path::exists(Path::new(&processing_unit.input)) {
        panic!("{:?}", &processing_unit.input);
    }

    let mut rdr = Reader::from_path(&processing_unit.input).unwrap();
    let headers: Vec<String> = rdr.headers()
        .unwrap()
        .iter()
        .map(|s| String::from(s).replace('\"', "\\\""))
        .collect();

    write_to_file(rdr, &headers, &processing_unit.output)

}


fn to_absolute(input: &OsStr, path: &Path) -> String {
    
    let input_str = input.to_str().unwrap();
    let last = input_str.split(MAIN_SEPARATOR).last().unwrap();
    let last_with_separator = format!("{}{}", String::from(MAIN_SEPARATOR), String::from(last));
    let prefix = input_str.replace(&last_with_separator, &String::from(""));
    format!(
        "{}/{}.{}",
        prefix,
        path.file_stem().unwrap().to_str().unwrap(),
        path.extension().unwrap().to_str().unwrap(),
    )
}

pub fn run_by_option(options: &ApplicationOptions) -> Result<(), Box<dyn Error>> {
    let files = collect_files(options);
    files.par_iter().for_each(convert_data);
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let options: ApplicationOptions = arg_parse();
    run_by_option(&options)
}
