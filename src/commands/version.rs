use super::command::Command;

pub struct Version {
    name: String,
}

impl Version {
    pub fn new() -> Version {
        Version {
            name: String::from("version"),
        }
    }
}

impl Command for Version {
    fn execute(&self) {
        // todo handle arg passed in
        println!("pokeman version: {}", env!("CARGO_PKG_VERSION"));
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn help(&self) -> &str {
        // todo proper help text
        "version\t-\tdisplay the installed version of pokeman"
    }
}