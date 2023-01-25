use super::command::Command;

pub struct Help {
    name: String,
    commands: Vec<Box<dyn Command>>,
    option: Option<Box<dyn Command>>,
}

impl Help {
    pub fn new (commands: Vec<Box<dyn Command>>, option: Option<Box<dyn Command>>) -> Help {
        Help {
            name: String::from("help"),
            commands,
            option,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Command for Help {
    fn execute(&self) {
        if let Some(arg) = &self.option {
            println!("{}", arg.help());
        } else {
            println!("{}", &self.help());
            // todo handle arg passed in
            for command in self.commands.iter() {
                println!("{}", command.help());
            }
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn help(&self) -> &str {
        // todo proper help text
        "help\t-\tdisplay help text for commands, shows all commands if none specified\n\t[COMMAND]\tshow help for a specific command"
    }
}