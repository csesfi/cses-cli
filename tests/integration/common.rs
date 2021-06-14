use predicates::str::{ContainsPredicate, RegexPredicate};
use std::path::Path;

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

pub fn contains(expr: &str) -> ContainsPredicate {
    predicate::str::contains(expr)
}

pub fn log_in() {
    command().args(&["login"]).assert().success();
    minreq::post("http://127.0.0.1:4011/authorize-all")
        .send()
        .unwrap();
}

pub fn create_file<P: AsRef<Path>, S: AsRef<[u8]>>(filename: P, content: S) {
    let mut file = std::fs::File::create(filename.as_ref()).unwrap();
    std::io::Write::write_all(&mut file, content.as_ref()).unwrap();
}
