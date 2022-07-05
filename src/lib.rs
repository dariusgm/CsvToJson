
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{MAIN_SEPARATOR, Path};

use csv::{Reader, StringRecord};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use parsing::ApplicationOptions;

mod parsing;

#[derive(Debug)]
pub struct ProcessingUnit {
    input: String,
    output: String
}


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

pub fn write_to_file(mut rdr: Reader<File>, headers: &Vec<String>, output: &String) {
    let mut file_handler = File::create(output).unwrap();
    rdr.records().for_each(|optional_record| {
        for record in optional_record {
            let converted_line_output = convert_line(&headers, &record);
            let _ = file_handler.write_all(converted_line_output.as_bytes());
        }
    });
}

pub fn write_to_stdout(mut rdr: Reader<File>, headers: &Vec<String>) {
    rdr.records().for_each(|optional_record| {
        let record = optional_record.unwrap();
        let converted_line_output = convert_line(headers, &record);
        println!("{}", converted_line_output);
    });
}

pub fn collect_files(options: &ApplicationOptions) -> Vec<ProcessingUnit> {
    let mut files_to_process = Vec::new();

    for argument in &options.input {
        for entry in glob::glob(argument).unwrap() {
            match entry {
                Ok(path) => {
                    let file_name = path.display();
                    // println!("{:?}", file_name);

                    let input = to_absolute(file_name.to_string(), &path);
                    let output = match &options.output {
                        None => format!("{}.json", file_name),
                        Some(x) => format!("{}/{}.json", x, file_name)
                    };


                    let processing_unit = ProcessingUnit {
                        input,
                        output
                    };

                    files_to_process.push(processing_unit)
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


fn to_absolute(input: String, path: &Path) -> String {
    let last = input.split(MAIN_SEPARATOR).last().unwrap();
    let last_with_separator = format!("{}{}", String::from(MAIN_SEPARATOR), String::from(last));
    let prefix = input.replace(&last_with_separator, &String::from(""));
    format!(
        "{}/{}.{}",
        prefix,
        path.file_stem().unwrap().to_str().unwrap(),
        path.extension().unwrap().to_str().unwrap(),
    )
}

pub fn run_by_str(args: Vec<&str>) -> Result<(), Box<dyn Error>> {
    run(args.iter().map(|&x| x.into()).collect())
}

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{:?}", args);
    let options = parsing::arg_parse();
    let files = collect_files(&options);
    files.par_iter().for_each(convert_data);
    Ok(())
}
