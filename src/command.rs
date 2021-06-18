use anyhow::{anyhow, Context, Result};

use crate::entities::Language;

pub static HELP_STR: &str = r#"CSES CLI

USAGE:
    cses-cli <command> [OPTIONS] [FLAGS]

FLAGS:
    -h, --help              Prints this help message.

COMMANDS:
    help                    Prints this help message.
    login                   Log in to cses.fi.
    logout                  Invalidate the current login session.
    status                  Prints the login status.
    courses                 Displays a list of courses.
    course <course-id>      Displays the contents of a course.
    submit [-ctlo] <file>   Submit a file to cses.fi.
        Task ID, language, and the language option may be automatically deduced
        by the server, in which case they do not need to be supplied as options.
    submissions [-c] (-t)   List previous submissions to a task.
    submission [-c] <id>    Show details about the submission with given ID.
    template [-cftl]        Download and save a code template from cses.fi
        The template will be saved to the current directory with a filename
        specified by the server. File, task ID and language are optional
        and will be used by the server to select a suitable code template.

OPTIONS:
    -c <course-id>, --course <course-id>
        Textual course ID, e.g. "problemset". Any previously supplied value is
        remembered.
    -t <task-id>, --task <task-id>
        Numeric task ID.
    -l <language>, --language <language>
        Specifies the programming language of the submitted file or the
        downloaded template.
    -o <language-option>, --option <language-option>
        Optionally specifies a language option. For example, the language "C++"
        has possible options "C++11" and "C++17".
    -f <file>, --file <file>
        Selects the template with filename `file`.
"#;

pub static NO_COMMAND_PROVIDED_HINT: &str = r#"No command provided. Run `help` 
to get a list of available commands."#;

pub static LANGUAGE_HINT: &str = r#"You can manually specify the language with
the `-l` or `--language` flags, e.g.:

cses-cli submit hello_world.rs -l Rust
"#;

pub static TASK_HINT: &str = r#"You can manually specify the task with
the `-t` or `--task` flags, e.g.:

cses-cli submit hello_world.rs -t 1337
"#;

type CourseId = Option<String>;

#[derive(Debug)]
pub enum Command {
    None,
    Help,
    Login,
    Logout,
    Status,
    Courses,
    Course(Course),
    Submit(Submit),
    Template(Template),
    Submissions(CourseId, u64),
    Submission(CourseId, u64),
}
#[derive(Debug)]
pub struct Submit {
    pub course_id: CourseId,
    pub task_id: Option<u64>,
    pub language: Language,
    pub file_name: String,
}
#[derive(Debug)]
pub struct Template {
    pub course_id: Option<String>,
    pub task_id: Option<u64>,
    pub language: Option<String>,
    pub file_name: Option<String>,
}
fn parse_course(pargs: &mut pico_args::Arguments) -> Result<Option<String>> {
    pargs
        .opt_value_from_str(["-c", "--course"])
        .context("Failed parsing course ID")
}
fn parse_task_id(pargs: &mut pico_args::Arguments) -> Result<Option<u64>> {
    Ok(pargs.opt_value_from_str(["-t", "--task"])?)
}
fn parse_language_name(pargs: &mut pico_args::Arguments) -> Result<Option<String>> {
    Ok(pargs.opt_value_from_str(["-l", "--language"])?)
}
fn parse_language_option(pargs: &mut pico_args::Arguments) -> Result<Option<String>> {
    Ok(pargs.opt_value_from_str(["-o", "--lang-opt"])?)
}
impl Submit {
    fn parse(pargs: &mut pico_args::Arguments) -> Result<Submit> {
        Ok(Submit {
            course_id: parse_course(pargs)?,
            task_id: parse_task_id(pargs)?,
            language: Language {
                name: parse_language_name(pargs)?,
                option: parse_language_option(pargs)?,
            },
            file_name: {
                if let Ok(file_name) = pargs.free_from_str() {
                    file_name
                } else {
                    anyhow::bail!("File name not specified")
                }
            },
        })
    }
}
impl Template {
    fn parse(pargs: &mut pico_args::Arguments) -> Result<Template> {
        Ok(Template {
            course_id: parse_course(pargs)?,
            task_id: parse_task_id(pargs)?,
            language: parse_language_name(pargs)?,
            file_name: pargs.opt_value_from_str(["-f", "--file"])?,
        })
    }
}
#[derive(Debug)]
pub struct Course {
    pub course_id: String,
}
impl Course {
    fn parse(pargs: &mut pico_args::Arguments) -> Result<Course> {
        Ok(Course {
            course_id: {
                if let Ok(course_id) = pargs.free_from_str() {
                    course_id
                } else {
                    anyhow::bail!("Course ID not specified")
                }
            },
        })
    }
}
impl Command {
    pub fn from_command_line() -> Result<Command> {
        let pargs = pico_args::Arguments::from_env();

        Command::parse_command(pargs)
    }

    fn parse_command(mut pargs: pico_args::Arguments) -> Result<Command> {
        if pargs.contains(["-h", "--help"]) {
            return Ok(Command::Help);
        }

        let command = pargs.subcommand()?.unwrap_or_default();
        let result = match command.as_str() {
            "" => Command::None,
            "help" => Command::Help,
            "login" => Command::Login,
            "logout" => Command::Logout,
            "status" => Command::Status,
            "courses" => Command::Courses,
            "course" => Command::Course(
                Course::parse(&mut pargs).context("Failed parsing command `Course`")?,
            ),
            "submit" => Command::Submit(
                Submit::parse(&mut pargs).context("Failed parsing command `Submit`")?,
            ),
            "template" => Command::Template(
                Template::parse(&mut pargs).context("Failed parsing command `template`")?,
            ),
            "submissions" => Command::Submissions(
                parse_course(&mut pargs)?,
                pargs
                    .value_from_str(["-t", "--task"])
                    .context("Failed parsing task ID")?,
            ),
            "submission" => Command::Submission(
                parse_course(&mut pargs)?,
                pargs
                    .free_from_str()
                    .context("Failed parsing submission ID")?,
            ),
            _ => return Err(anyhow!("Invalid command: {}", command)),
        };

        let unused_args = pargs.finish();
        if !unused_args.is_empty() {
            return Err(anyhow!("Unused arguments: {:?}", unused_args))
                .context(format!("Could not parse command `{}`", command));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_pargs(args: &[&str]) -> pico_args::Arguments {
        pico_args::Arguments::from_vec(args.iter().map(|s| s.to_string().into()).collect())
    }

    #[test]
    fn unknown_command_is_invalid() {
        let pargs = to_pargs(&["asdf"]);
        let command = Command::parse_command(pargs);

        assert!(command.is_err());
    }

    #[test]
    fn unknown_command_is_valid_with_h_flag() {
        let pargs = to_pargs(&["-h"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Help));
    }

    #[test]
    fn unknown_command_is_valid_with_help_flag() {
        let pargs = to_pargs(&["--help"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Help));
    }

    #[test]
    fn command_help_no_flags() {
        let pargs = to_pargs(&["help"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Help));
    }

    #[test]
    fn command_login_no_flags() {
        let pargs = to_pargs(&["login"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Login));
    }

    #[test]
    fn logout_without_flags_returns_logout() {
        let pargs = to_pargs(&["logout"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Logout));
    }

    #[test]
    fn logout_with_h_flag_returns_help() {
        let pargs = to_pargs(&["logout", "-h"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Help));
    }

    #[test]
    fn command_submit_with_file_no_flags() {
        let pargs = to_pargs(&["submit", "test.cpp"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Submit(_)));
    }

    #[test]
    fn submit_returns_the_correct_file_string() {
        let pargs = to_pargs(&["submit", "test.cpp"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(submit)
            if submit.file_name == "test.cpp"
        ));

        let pargs = to_pargs(&["submit", "qwerty.rs"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(submit)
            if submit.file_name == "qwerty.rs"
        ));
    }

    #[test]
    fn submit_course_long_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "--course", "alon"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(Submit { course_id: Some(course), .. })
            if course == "alon"
        ));
    }

    #[test]
    fn submit_course_short_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "-c", "alon"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(Submit { course_id: Some(course), .. })
            if course == "alon"
        ));
    }

    #[test]
    fn submit_task_long_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "--task", "123"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(Submit { task_id: Some(task), .. })
            if task == 123
        ));
    }

    #[test]
    fn submit_task_short_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "-t", "123"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(Submit { task_id: Some(task), .. })
            if task == 123
        ));
    }

    #[test]
    fn submit_task_id_should_be_integer() {
        let pargs = to_pargs(&["submit", "test.cpp", "--task", "asdf"]);
        assert!(Command::parse_command(pargs).is_err());
        let pargs = to_pargs(&["submit", "test.cpp", "-t", "asdf"]);
        assert!(Command::parse_command(pargs).is_err());
    }

    #[test]
    fn submit_language_long_name_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "--language", "Rust"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(Submit { language: Language { name: Some(lang), ..}, .. })
            if lang == "Rust"
        ));
    }

    #[test]
    fn submit_language_short_name_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "-l", "Rust"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(Submit { language: Language { name: Some(lang), .. }, .. })
            if lang == "Rust"
        ));
    }

    #[test]
    fn submit_language_option_long_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "--lang-opt", "C++17"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(Submit { language: Language{ option: Some(opt), .. }, .. })
            if opt == "C++17"
        ));
    }
    #[test]
    fn submit_language_option_short_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "-o", "C++17"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(Submit { language: Language{ option: Some(opt), .. }, ..})
            if opt == "C++17"
        ));
    }

    #[test]
    fn command_submit_fails_without_a_file() {
        let pargs = to_pargs(&["submit"]);
        let command = Command::parse_command(pargs);

        assert!(command.is_err());
    }

    #[test]
    fn no_command_no_flags() {
        let pargs = to_pargs(&[]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::None));
    }

    #[test]
    fn command_courses_works_without_flags() {
        let pargs = to_pargs(&["courses"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Courses));
    }

    #[test]
    fn command_template_works_with_language_and_task() {
        let pargs = to_pargs(&["template", "-c", "course", "-t", "123", "-l", "language"]);
        let command = Command::parse_command(pargs).unwrap();
        assert!(matches!(command, Command::Template(
        Template {
            course_id: Some(course_id),
            task_id: Some(task_id),
            language: Some(language),
            file_name: None,
        }) if course_id == "course" && task_id == 123 && language == "language"
        ));
    }
    #[test]
    fn command_template_works_without_course_id() {
        let pargs = to_pargs(&["template", "-t", "123", "-l", "language"]);
        let command = Command::parse_command(pargs).unwrap();
        assert!(matches!(command, Command::Template(
        Template {
            course_id: None,
            task_id: Some(task_id),
            language: Some(language),
            file_name: None,
        }) if task_id == 123 && language == "language"
        ));
    }
    #[test]
    fn command_template_works_with_file_name() {
        let pargs = to_pargs(&["template", "-c", "course", "-f", "file"]);
        let command = Command::parse_command(pargs).unwrap();
        assert!(matches!(command, Command::Template(
        Template {
            course_id: Some(course_id),
            task_id: None,
            language: None,
            file_name: Some(file_name),
        }) if course_id == "course" && file_name == "file"
        ));
    }
    #[test]
    fn command_template_works_with_long_parameters_task_language() {
        let pargs = to_pargs(&[
            "template",
            "--course",
            "course",
            "--task",
            "123",
            "--language",
            "language",
        ]);
        let command = Command::parse_command(pargs).unwrap();
        assert!(matches!(command, Command::Template(
        Template {
            course_id: Some(course_id),
            task_id: Some(task_id),
            language: Some(language),
            file_name: None,
        }) if course_id == "course" && task_id == 123 && language == "language"
        ));
    }
    #[test]
    fn command_template_works_with_long_parameters_file_name() {
        let pargs = to_pargs(&["template", "--file", "file"]);
        let command = Command::parse_command(pargs).unwrap();
        assert!(matches!(command, Command::Template(
        Template {
            course_id: None,
            task_id: None,
            language: None,
            file_name: Some(file_name),
        }) if file_name == "file"
        ));
    }
    #[test]
    fn submissions_without_course_parsed() {
        let pargs = to_pargs(&["submissions", "-t", "140"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Submissions(None, 140)));
    }

    #[test]
    fn submissions_with_course_parsed() {
        let pargs = to_pargs(&["submissions", "--task", "140", "-c", "alon"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submissions(Some(course), 140)
            if course == "alon"
        ));
    }

    #[test]
    fn submissions_fails_without_task() {
        let pargs = to_pargs(&["submissions", "-c", "alon"]);
        let command = Command::parse_command(pargs);
        assert!(command.is_err());
    }

    #[test]
    fn submission_without_course_parsed() {
        let pargs = to_pargs(&["submission", "1512"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Submission(None, 1512)));
    }

    #[test]
    fn unused_command_line_parameters_cause_an_error() {
        let pargs = to_pargs(&["help", "-c", "alon"]);
        assert!(Command::parse_command(pargs).is_err());
    }
}
