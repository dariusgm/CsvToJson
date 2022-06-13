use std::fmt::{Debug, Formatter};

pub struct ApplicationOptions {
    pub input: String,
    pub output: String,
    pub quiet: bool,
}

impl Clone for ApplicationOptions {
    fn clone(&self) -> Self {
        ApplicationOptions {
            input: self.input.clone(),
            output: self.output.clone(),
            quiet: self.quiet.clone(),
        }
    }
}


impl Default for ApplicationOptions {
    fn default() -> Self {
        Self {
            input: String::from("*.csv"),
            output: String::from(""),
            quiet: false,
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
