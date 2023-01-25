use commands::{command::Command, version::Version, generation::Generation, help::Help};

pub mod commands;

pub fn build_command(name: &str, option: Option<Box<dyn Command>>) -> Result<Box<dyn Command>, String> {
    match name {
        "help" => {
            let commands: Vec<Box<dyn Command>> = vec![
                Box::new(Version::new()),
                Box::new(Generation::new(None))
            ];
            return Ok(Box::new(Help::new(commands, option)));
        },
        "version" => {
            return Ok(Box::new(Version::new()));
        },
        "generation" => {
            return Ok(Box::new(Generation::new(option)));
        }
        _ => Err(format!("Command '{}' not found, see 'pokeman help' for list of commands", name))
    }
}