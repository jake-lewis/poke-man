pub struct Arg {
    pub name: String,
    pub option: Option<Box<dyn Command>>,
}

pub trait Command {
    fn execute(&self);
    fn name(&self) -> &str;
    fn help(&self) -> &str; // todo maybe make this it's own trait, bc help might not need it
}