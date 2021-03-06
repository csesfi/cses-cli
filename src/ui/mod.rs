mod courses;
mod login;
mod statement;
mod submission;
mod submissions;
mod table;
mod template;
mod test_case;
mod util;

use anyhow::{anyhow, Context, Error, Result};
use console::Term;

use crate::api::ApiError;
use crate::command::{ScopedCommand, Submission, HELP_STR, LANGUAGE_HINT, TASK_HINT};
use crate::{service, Command, Resources, ResourcesProvider};

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
        match command {
            Command::None => {
                self.term.write_line(HELP_STR)?;
                login::status(self).context("Could not get login status")?;
            }
            Command::Help => {
                self.term.write_str(HELP_STR)?;
            }
            Command::Login => {
                login::login(self)?;
            }
            Command::Logout => {
                login::logout(self)?;
            }
            Command::Status => {
                login::status(self)?;
            }
            Command::Courses => {
                courses::list_courses(self)?;
            }
            Command::Scoped(scope, command) => {
                let scope = service::select_scope(&mut self.res, scope)?;
                match command {
                    ScopedCommand::List => {
                        courses::list_content(self, &scope)?;
                    }
                    ScopedCommand::Submit(submit) => {
                        let submission_info = service::submit(&mut self.res, &scope, submit)?;
                        submission::print_submission_info(self, &scope, submission_info, true)?;
                    }
                    ScopedCommand::Template(template) => {
                        template::get_template(self, &scope, template)?;
                    }
                    ScopedCommand::Submissions(task_id) => {
                        submissions::list(self, &scope, &task_id)?;
                    }
                    ScopedCommand::Submission(submission) => {
                        let submission_info = match submission {
                            Submission::Id(submission_id) => service::submission_info(
                                &mut self.res,
                                &scope,
                                submission_id,
                                false,
                            )?,
                            Submission::NthLatest(task_id, n) => {
                                service::nth_latest_submission_info(
                                    &mut self.res,
                                    &scope,
                                    &task_id,
                                    n,
                                )?
                            }
                        };
                        submission::print_submission_info(self, &scope, submission_info, false)?;
                    }
                    ScopedCommand::View(task_id) => {
                        let task_statement =
                            service::get_task_statement(&self.res, &scope, &task_id)?;
                        statement::print_statement(self, &task_statement)?;
                    }
                    ScopedCommand::Samples(samples) => {
                        test_case::get_samples(self, &scope, samples)?;
                    }
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                return Err(anyhow!("Command not yet implemented"));
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

    #[allow(unused)]
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
        let indentation = "    ";
        let prefix = if i == 0 {
            "".to_owned()
        } else {
            indentation.to_owned()
        };
        println!("{}", add_indentation(&error.to_string(), &prefix));
        if let Some(hint) = get_error_hint(error) {
            let prefix = prefix.to_owned() + indentation;
            println!("{}\n", add_indentation("Hint:", &prefix));
            println!("{}", add_indentation(hint, &prefix));
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
        result.push_str(line);
    }
    result
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
