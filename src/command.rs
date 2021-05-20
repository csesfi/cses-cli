use anyhow::Result;

pub static HELP_STR: &str = r#"CSES CLI

USAGE:
    cses-cli <command>

COMMANDS:
    help                Prints this help message.
"#;

pub enum Command {
    Help,
}

impl Command {
    pub fn from_command_line() -> Result<Command> {
        Ok(Command::Help)
    }
}
