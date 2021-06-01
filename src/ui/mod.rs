mod login;
mod submit;

use anyhow::{Error, Result};
use console::Term;

use crate::command::HELP_STR;
use crate::service;
use crate::{Command, Resources, ResourcesProvider};

pub struct Ui<R: ResourcesProvider> {
    res: Resources<R>,
    term: Term,
}

impl<R: ResourcesProvider> Ui<R> {
    pub fn with_resources(res: Resources<R>) -> Self {
        let term = Term::stdout();
        Ui { res, term }
    }

    pub fn run(&mut self, command: Command) -> Result<()> {
        service::ping(&mut self.res);
        match command {
            Command::Help => {
                self.term.write_str(HELP_STR)?;
            }
            Command::Login => {
                login::login(self)?;
            }
            Command::Logout => {
                login::logout(self)?;
            }
            Command::Submit(submit) => {
                submit::submit(self, submit)?;
            }
            _ => {
                self.term.write_line("Command not yet implemented")?;
            }
        }
        Ok(())
    }
}

pub fn print_error(err: &Error) {
    println!("{:?}", err);
}
