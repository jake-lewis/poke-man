use super::command::{Command, Arg};

pub struct Help {
    name: String,
    commands: Vec<Box<dyn Command>>,
}

impl Help {
    pub fn new(commands: Vec<Box<dyn Command>>) -> Self {
        Self {
            name: String::from("help"),
            commands
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Command for Help {
    fn execute(&self, arg: Option<&Arg>) {
        // todo handle arg passed in
        for command in self.commands.iter() {
            println!("{}", command.help());
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn help(&self) -> &str {
        // todo proper help text
        "todo help help text"
    }
}