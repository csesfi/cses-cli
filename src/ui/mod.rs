mod login;
mod submission;

use anyhow::{Error, Result};
use console::{Style, Term};

use crate::command::HELP_STR;
use crate::entities::{Language, SubmissionInfo, SubmissionTestInfo};
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
            Command::Submit(submit) => {
                service::update_submit_parameters(&mut self.res, &submit)?;
                let submission_id = service::submit(&mut self.res, submit.file_name)?;
                let long_poll = false;
                submission::print_submission_info(self, submission_id, long_poll)?;
            }
            _ => {
                submission::print_submission_info(self, 1, true)?;
            }
        }
        Ok(())
    }
}

pub fn print_error(err: &Error) {
    println!("{:?}", err);
}

pub fn print_with_color(line: String) {
    let mut color = Style::new().red();
    if line == "ACCEPTED" {
        color = Style::new().green();
    }
    print!("{}", color.apply_to(line));
}
