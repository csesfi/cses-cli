use anyhow::Result;
use console::Term;

use crate::command::HELP_STR;
use crate::service;
use crate::{Command, Resources, ResourcesProvider};

use crate::login::Login;

#[allow(unused)] // FIXME
pub struct Ui<R: ResourcesProvider> {
    res: Resources<R>,
    term: Term,
}

static USERNAME_STR: &str = "Username: ";
static PASSWORD_STR: &str = "Password: ";

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
                self.login()?;
            }
            _ => {
                self.term.write_str("Command not yet implemented\n")?;
            }
        }
        Ok(())
    }

    fn login(&mut self) -> Result<()> {
        let login = Login {
            username: self.prompt_username()?,
            password: self.prompt_password()?,
        };
        service::login(&mut self.res, &login)
    }
    fn prompt_username(&mut self) -> Result<String> {
        self.term.write_str(USERNAME_STR)?;
        Ok(self.term.read_line()?)
    }
    fn prompt_password(&mut self) -> Result<String> {
        self.term.write_str(PASSWORD_STR)?;
        Ok(self.term.read_secure_line()?)
    }
}
