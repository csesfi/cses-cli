mod login;

use anyhow::{Error, Result};
use console::Term;

use crate::command::HELP_STR;
use crate::service;
use crate::{Command, Resources, ResourcesProvider};

pub struct Ui<R: ResourcesProvider> {
    res: Resources<R>,
    term: Term,
    raw_stdin: bool,
}

impl<R: ResourcesProvider> Ui<R> {
    pub fn with_resources(raw_stdin: bool, res: Resources<R>) -> Self {
        let term = Term::stdout();
        Ui {
            res,
            term,
            raw_stdin,
        }
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
                service::logout(&mut self.res)?;
                self.term.write_line("Login invalidated successfully")?;
            }
            _ => {
                self.term.write_line("Command not yet implemented")?;
            }
        }
        Ok(())
    }

    fn prompt_line(&self) -> Result<String> {
        if self.raw_stdin {
            // Copied from the console crate
            let mut rv = String::new();
            std::io::stdin().read_line(&mut rv)?;
            let len = rv.trim_end_matches(&['\r', '\n'][..]).len();
            rv.truncate(len);
            Ok(rv)
        } else {
            Ok(self.term.read_line()?)
        }
    }

    fn prompt_secure_line(&self) -> Result<String> {
        if self.raw_stdin {
            self.prompt_line()
        } else {
            Ok(self.term.read_secure_line()?)
        }
    }
}

pub fn print_error(err: &Error) {
    println!("{:?}", err);
}
