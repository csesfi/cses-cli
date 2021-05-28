use predicates::str::RegexPredicate;

pub use assert_cmd::prelude::*;
pub use assert_cmd::Command;
pub use linkme::distributed_slice;
pub use predicates::prelude::*;

#[distributed_slice]
pub static TESTS: [fn()] = [..];

pub fn command() -> Command {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.env("CSES_CLI_TEST", "true");
    command
}

pub fn regex_match(expr: &str) -> RegexPredicate {
    predicate::str::is_match(expr).unwrap()
}
