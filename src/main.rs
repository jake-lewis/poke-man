use std::{env, process};

use poke_man::{commands::command::Command, build_command};

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