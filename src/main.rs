use std::{env, process};

use poke_man::commands::{
    command::Command, 
    help::Help, 
    config::Config, 
    version::Version
};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Must provide an argument, see 'pokeman help' for more info");
        process::exit(1);
    }
    
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
    if args.len() == 1 {
       return build_command(&name, None)
    }
    
    let arg = parse_args(&args[1..]);

    match arg {
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
            let commands: Vec<Box<dyn Command>> = vec![Box::new(Version::new()), Box::new(Config::new())];
            return Ok(Box::new(Help::new(commands, option)));
        },
        "config" => {
            return Ok(Box::new(Config::new()));
        },
        "version" => {
            return Ok(Box::new(Version::new()));
        },
        _ => Err(format!("Command '{}' not found, see 'pokeman help' for list of commands", name))
    }
}