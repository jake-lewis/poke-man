use super::command::{Command, Arg};

pub struct Config {
    name: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            name: String::from("config"),
        }
    }
}

impl Command for Config {
    fn execute(&self, arg: Option<&Arg>) {
        // todo handle arg passed in
        eprintln!("not implemented yet");
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn help(&self) -> &str {
        // todo proper help text
        "todo config help text"
    }
}