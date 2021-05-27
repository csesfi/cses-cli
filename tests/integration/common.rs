use predicates::str::RegexPredicate;
use std::process::Command;

pub use assert_cmd::prelude::*;
pub use linkme::distributed_slice;
pub use predicates::prelude::*;

#[distributed_slice]
pub static TESTS: [fn()] = [..];

pub fn command() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

pub fn regex_match(expr: &str) -> RegexPredicate {
    predicate::str::is_match(expr).unwrap()
}
