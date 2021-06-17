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

OPTIONS:
    -c <course-id>, --course <course-id>
        Textual course ID, e.g. "problemset". Any previously supplied value is
        remembered.
    -t <task-id>, --task <task-id>
        Numeric task ID.
    -l <language>, --language <language>
        Specifies the programming language of the submitted file.
    -o <language-option>, --option <language-option>
        Optionally specifies a language option. For example, the language "C++"
        has possible options "C++11" and "C++17".
                                    
"#;

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
    Submissions(CourseId, u64),
    Submission(CourseId, u64),
}
fn parse_course(pargs: &mut pico_args::Arguments) -> Result<Option<String>> {
    pargs
        .opt_value_from_str(["-c", "--course"])
        .context("Failed parsing course ID")
}
#[derive(Debug)]
pub struct Submit {
    pub course_id: CourseId,
    pub task_id: Option<u64>,
    pub language: Language,
    pub file_name: String,
}
impl Submit {
    fn parse(pargs: &mut pico_args::Arguments) -> Result<Submit> {
        Ok(Submit {
            course_id: parse_course(pargs)?,
            task_id: pargs.opt_value_from_str(["-t", "--task"])?,
            language: Language {
                name: pargs.opt_value_from_str(["-l", "--language"])?,
                option: pargs.opt_value_from_str(["-o", "--lang-opt"])?,
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
        Ok(match command.as_str() {
            "" => Command::None,
            "help" => Command::Help,
            "login" => Command::Login,
            "logout" => Command::Logout,
            "status" => Command::Status,
            "courses" => Command::Courses,
            "course" => Command::Course(
                Course::parse(&mut pargs).context("Failed parsing command `course`")?,
            ),
            "submit" => Command::Submit(
                Submit::parse(&mut pargs).context("Failed parsing command `submit`")?,
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
        })
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
}
