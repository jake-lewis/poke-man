use std::{env, process};

use poke_man::commands::{
    command::Command, 
    help::Help,
    version::Version, game::Game
};

fn main() {
    // Get input from cmd line
    let args: Vec<String> = env::args().collect();
    
    // Check if there are enough args
    if args.len() < 2 {
        eprintln!("Must provide an argument, see 'pokeman help' for more info");
        process::exit(1);
    }
    
    // Build top level arg
    match parse_args(&args[1..]) {
        Ok(command) => command.execute(),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}

fn parse_args(args: &[String]) -> Result<Box<dyn Command>, String> {
    let name = args[0].clone();

    // No subcommands
    if args.len() == 1 {
        return build_command(&name, None)
    }
    
    match parse_args(&args[1..]) {
        Ok(cmd) => {
            match build_command(&name, Some(cmd)) {
                Ok(command) => Ok(command),
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}

fn build_command(name: &str, option: Option<Box<dyn Command>>) -> Result<Box<dyn Command>, String> {
    match name {
        "help" => {
            let commands: Vec<Box<dyn Command>> = vec![
                Box::new(Version::new()),
                Box::new(Game::new(None))
            ];
            return Ok(Box::new(Help::new(commands, option)));
        },
        "version" => {
            return Ok(Box::new(Version::new()));
        },
        "game" => {
            return Ok(Box::new(Game::new(option)));
        }
        _ => Err(format!("Command '{}' not found, see 'pokeman help' for list of commands", name))
    }
}