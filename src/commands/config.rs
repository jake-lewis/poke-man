use super::command::Command;

pub struct Config {
    name: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            name: String::from("config"),
        }
    }
}

impl Command for Config {
    fn execute(&self) {
        // todo handle arg passed in
        eprintln!("not implemented yet");
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn help(&self) -> &str {
        // todo proper help text
        "config\t-\tconfigure pokeman user settings"
    }
}