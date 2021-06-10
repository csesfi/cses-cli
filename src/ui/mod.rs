mod login;
mod submission;
mod submit;

use anyhow::{Error, Result};
use console::{Style, Term};

use crate::api::ApiError;
use crate::command::{HELP_STR, LANGUAGE_HINT, TASK_HINT};
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
                login::logout(self)?;
            }
            Command::Submit(submit) => {
                let submission_response = submit::submit(self, submit)?;
                submission::print_submission_info(self, submission_response.submission_id, true)?;
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
    for (i, error) in err.chain().enumerate() {
        let prefix = if i == 0 {
            "".to_owned()
        } else {
            "\t".to_owned()
        };
        println!("{}", add_indentation(&error.to_string(), &prefix));
        if let Some(hint) = get_error_hint(error) {
            let prefix = prefix.to_owned() + "\t";
            println!("{}\n", add_indentation("Hint:", &prefix));
            println!("{}", add_indentation(&hint, &prefix));
        }
    }
}

fn get_error_hint(error: &(dyn std::error::Error + 'static)) -> Option<&'static str> {
    match error.downcast_ref::<ApiError>() {
        Some(ApiError::LanguageDeductionError(_)) => Some(LANGUAGE_HINT),
        Some(ApiError::TaskDeductionError(_)) => Some(TASK_HINT),
        _ => None,
    }
}

fn add_indentation(text: &str, prefix: &str) -> String {
    let mut result = String::new();
    for line in text.split_inclusive('\n') {
        result.push_str(prefix);
        result.push_str(&line);
    }
    result
}

pub fn print_with_color(line: String) {
    let mut color = Style::new().red();
    if line == "ACCEPTED" {
        color = Style::new().green();
    }
    print!("{}", color.apply_to(line));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "ei\noppi\nojaan\nkaada";

    #[test]
    fn test_add_indentation_empty_prefix_does_noting() {
        assert_eq!(add_indentation(&TEST, ""), TEST);
    }
    #[test]
    fn test_add_indentation_simple() {
        assert_eq!(add_indentation(&TEST, " "), " ei\n oppi\n ojaan\n kaada");
    }
}
