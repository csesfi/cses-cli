use anyhow::Result;
use console::Term;

use crate::command::HELP_STR;
use crate::{Command, Service};

#[allow(unused)] // FIXME
pub struct Ui<S> {
    service: S,
    term: Term,
}

impl<S: Service> Ui<S> {
    pub fn with_service(service: S) -> Self {
        let term = Term::stdout();
        Ui { service, term }
    }

    pub fn run(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Help => {
                self.term.write_str(HELP_STR)?;
            }
        }
        Ok(())
    }
}
