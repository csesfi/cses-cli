use anyhow::{anyhow, Result};

pub static HELP_STR: &str = r#"CSES CLI

USAGE:
    cses-cli <command>

FLAGS:
    -h, --help          Prints this help message.

COMMANDS:
    help                Prints this help message.
    login               Log in to cses.fi
    submit <file>       Submit a file to cses.fi.
"#;

#[derive(Debug)]
pub enum Command {
    None,
    Help,
    Login,
    Submit(String),
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
            "submit" => {
                let file_name = pargs.free_from_str::<String>();
                match file_name {
                    Ok(file_name) => Ok(Command::Submit(file_name)),
                    Err(_) => Err(anyhow!("File name not specified")),
                }
            }
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
    fn command_submit_with_file_no_flags() {
        let pargs = to_pargs(&["submit", "test.cpp"]);
        let command = Command::parse_command(pargs).unwrap();

        assert!(matches!(command, Command::Submit(_)));
    }

    #[test]
    fn submit_returns_the_correct_file_string() {
        let pargs = to_pargs(&["submit", "test.cpp"]);
        let command = Command::parse_command(pargs).unwrap();

        if let Command::Submit(file_name) = command {
            assert_eq!(file_name, String::from("test.cpp"))
        };

        let pargs = to_pargs(&["submit", "qwerty.rs"]);
        let command = Command::parse_command(pargs).unwrap();

        if let Command::Submit(file_name) = command {
            assert_eq!(file_name, String::from("qwerty.rs"))
        }
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
