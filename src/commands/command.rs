pub struct Arg {
    pub name: String,
    pub option: Option<Box<Arg>>,
}

pub trait Command {
    fn execute(&self, arg: Option<&Arg>);
    fn name(&self) -> &str;
    fn help(&self) -> &str; // todo maybe make this it's own trait, bc help might not need it
}