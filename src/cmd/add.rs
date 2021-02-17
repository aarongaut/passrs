use super::{Command, CommandWrapper};
use crate::db;
use crate::cli;

pub struct ArgsAdd {
    show: bool,
}

pub struct CommandAdd {
}

impl Command for CommandAdd {
    type Args = ArgsAdd;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandAdd {})
    }
    fn name(&self) -> &'static str { "add" }
    fn help(&self) -> &'static str { "Add a new entry" }
    fn run(&self, opts: ArgsAdd, db: &mut db::Database) {
        if opts.show {
            println!("adding");
        }
        let input = cli::read_editor("", "Enter any notes above");
        if let Ok(text) = input {
            println!("{}", text);
        }
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> ArgsAdd {
        ArgsAdd { show: true }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('A')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
