use std::fmt::{Debug, Formatter};
use clap::Parser;
use log::info;
use crate::ApplicationOptions;


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