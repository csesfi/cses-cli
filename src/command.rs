use anyhow::{anyhow, Context, Result};

pub static HELP_STR: &str = r#"CSES CLI

USAGE:
    cses-cli <command> [OPTIONS] [FLAGS]

FLAGS:
    -h, --help          Prints this help message.

COMMANDS:
    help                Prints this help message.
    login               Log in to cses.fi
    logout              Invalidate the current login session.
    submit <file>       Submit a file to cses.fi.

OPTIONS:
    --course-id COURSE_ID
    --task-id TASK_ID
    --language LANGUAGE
    --lang-opt LANGUAGE_OPTIONS
"#;

#[derive(Debug)]
pub enum Command {
    None,
    Help,
    Login,
    Logout,
    Submit(Submit),
}
#[derive(Debug)]
pub struct Submit {
    course_id: Option<String>,
    task_id: Option<u64>,
    language_name: Option<String>,
    language_option: Option<String>,
    file_name: String,
}
impl Submit {
    fn parse(pargs: &mut pico_args::Arguments) -> Result<Submit> {
        Ok(Submit {
            course_id: pargs.opt_value_from_str("--course-id")?,
            task_id: pargs.opt_value_from_str("--task-id")?,
            language_name: pargs.opt_value_from_str("--language")?,
            language_option: pargs.opt_value_from_str("--lang-opt")?,
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
        match command.as_str() {
            "" => Ok(Command::None),
            "help" => Ok(Command::Help),
            "login" => Ok(Command::Login),
            "logout" => Ok(Command::Logout),
            "submit" => Ok(Command::Submit(
                Submit::parse(&mut pargs).context("Failed parsing command `Submit`")?,
            )),
            _ => Err(anyhow!("Invalid command: {}", command)),
        }
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
    fn submit_course_id_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "--course-id", "alon"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(submit)
            if submit.course_id == Some("alon".to_string())
        ));
    }

    #[test]
    fn submit_task_id_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "--task-id", "123"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(submit)
            if submit.task_id == Some(123)
        ));
    }

    #[test]
    fn submit_task_id_should_be_integer() {
        let pargs = to_pargs(&["submit", "test.cpp", "--task-id", "asdf"]);

        assert!(matches!(Command::parse_command(pargs), Err(_)));
    }

    #[test]
    fn submit_language_name_parsed_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "--language", "Rust"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(submit)
            if submit.language_name== Some("Rust".to_string())
        ));
    }

    #[test]
    fn submit_language_option_correctly() {
        let pargs = to_pargs(&["submit", "test.cpp", "--lang-opt", "C++17"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(
            command,
            Command::Submit(submit)
            if submit.language_option== Some("C++17".to_string())
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
}
