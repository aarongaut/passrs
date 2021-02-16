use super::{Command, CommandWrapper};

pub struct ArgsYank {
    index: u32,
}

pub struct CommandYank {
}

impl Command for CommandYank {
    type Args = ArgsYank;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandYank {})
    }
    fn name(&self) -> &'static str { "yank" }
    fn help(&self) -> &'static str { "Copy the username/password to clipboard" }
    fn run(&self, opts: ArgsYank) {
    }
    fn parse(&self, raw_args: &clap::ArgMatches) -> ArgsYank {
        ArgsYank { index: 0 }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('Y')
    }
    fn repl_only(&self) -> bool {
        false
    }
}