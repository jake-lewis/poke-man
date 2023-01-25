use super::command::Command;

pub struct Game {
    name: String,
    option: Option<Box<dyn Command>>,
}

impl Game {
    pub fn new(option: Option<Box<dyn Command>>) -> Game {
        Game {
            name: String::from("game"),
            option
        }
    }
}

impl Command for Game {
    fn execute(&self) {
        // todo handle arg passed in
        eprintln!("not implemented yet");
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn help(&self) -> &str {
        "game\t-\tset the game to play\n\t[OPTION]\tthe default game used by other commands\n\tlist\t-\tlists the available games"
    }
}