use std::fmt::{Debug, Formatter};
use clap::Parser;
use log::info;

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


impl Clone for ApplicationOptions {
    fn clone(&self) -> Self {
        ApplicationOptions {
            input: self.input.clone(),
            output: self.output.clone(),
        }
    }
}


impl Default for ApplicationOptions {
    fn default() -> Self {
        Self {
            input: vec!["".to_owned()],
            output: None,
        }
    }
}


impl Debug for ApplicationOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.input)
            .field(&self.output)
            .finish()
    }
}

pub fn arg_parse() -> ApplicationOptions {
    env_logger::init();

    let cli = ApplicationOptions::parse();

    info!("Parsed following arguments: ");
    info!("input: {:?}", &cli.input);
    info!("output: {:?}", &cli.output);
    cli
}