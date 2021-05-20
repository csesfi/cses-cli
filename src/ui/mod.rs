use anyhow::Result;
use console::Term;

use crate::command::HELP_STR;
use crate::service;
use crate::{Command, Resources, ResourcesProvider};

#[allow(unused)] // FIXME
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
        }
        Ok(())
    }
}
