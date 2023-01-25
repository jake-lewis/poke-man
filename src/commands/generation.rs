use super::command::Command;

pub struct Generation {
    name: String,
    option: Option<Box<dyn Command>>,
}

impl Generation {
    pub fn new(option: Option<Box<dyn Command>>) -> Generation {
        Generation {
            name: String::from("generation"),
            option
        }
    }
}

impl Command for Generation {
    fn execute(&self) {
        // todo handle arg passed in
        eprintln!("not implemented yet");
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn help(&self) -> &str {
        "generation\t-\tset the generation to use\n\t[OPTION]\tthe default generation used by other commands\n\tlist\t-\tlists the available generations"
    }
}