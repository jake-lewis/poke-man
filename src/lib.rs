pub struct Arg {
    pub name: String,
    pub option: Option<Box<Arg>>,
}

pub fn match_cmd(arg_chain: &Arg) {
    match arg_chain.name.as_str() {
        "help" => {
            println!("pfft good luck");
        },
        "version" => {
            println!("pokeman version: {}", env!("CARGO_PKG_VERSION"));
        },
        _ => {
            eprintln!("Command {} not found, see 'pokeman help' for list of commands", arg_chain.name);
        }
    }
}
