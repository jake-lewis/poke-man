use commands::{version::Version, help::Help, command::{Command, Arg}, config::Config};

pub mod commands;

pub fn match_cmd(arg_chain: &Arg) {

    let commands: Vec<Box<dyn Command>> = vec![Box::new(Version::new()), Box::new(Config::new())];
    
    for command in commands.iter() {
        if command.name() == arg_chain.name.as_str() {
            command.execute(Some(arg_chain));
            return;
        }
    }

    let help = Help::new(commands);
    
    if help.name() == arg_chain.name.as_str() {
        help.execute(None)
    } else {
        eprintln!("Command {} not found, see 'pokeman help' for list of commands", arg_chain.name);
    }
}
