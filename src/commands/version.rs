use super::command::{Command, Arg};

pub struct Version {
    name: String,
}

impl Version {
    pub fn new() -> Self {
        Self {
            name: String::from("version"),
        }
    }
}

impl Command for Version {
    fn execute(&self, arg: Option<&Arg>) {
        // todo handle arg passed in
        println!("pokeman version: {}", env!("CARGO_PKG_VERSION"));
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn help(&self) -> &str {
        // todo proper help text
        "todo version help text"
    }
}