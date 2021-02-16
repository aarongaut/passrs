use super::{Command, CommandWrapper};

pub struct CommandHelp {
}

impl Command for CommandHelp {
    type Args = ();
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandHelp {})
    }
    fn name(&self) -> &'static str { "help" }
    fn help(&self) -> &'static str { "Print a help message" }
    fn run(&self, _: ()) {}
    fn parse(&self, raw_args: &clap::ArgMatches) -> () {}
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('H')
    }
    fn repl_only(&self) -> bool {
        true
    }
}