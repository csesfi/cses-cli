mod login;

use anyhow::Result;
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
                service::logout(&mut self.res)?;
                self.term.write_line("Login invalidated successfully")?;
            }
            Command::Submit(_submit) => {
                let submission_id = service::submit()?;
                let long_poll = false;
                let submission_info = service::submission(&mut self.res, submission_id, long_poll)?;
                self.term.write_line(&submission_info.status)?;
            }
            _ => {
                self.term.write_line("Command not yet implemented")?;
            }
        }
        Ok(())
    }
}
