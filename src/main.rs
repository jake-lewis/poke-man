use std::{env, process};

use poke_man::{Arg, match_cmd};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Must provide an argument, see 'pokeman help' for more info");
        process::exit(1);
    }
    
    let arg_chain: Arg = parse_args(&args[1..]);
    match_cmd(&arg_chain);
}

fn parse_args(args: &[String]) -> Arg {
    let name = args[0].clone();
    if args.len() == 1 {
       return Arg {
            name,
            option: None
        }
    }
    
    let arg = parse_args(&args[1..]);

    Arg {
        name,
        option: Some(Box::new(arg))
    }
}