use std::path::Path;
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

pub fn log_in(user: &str) {
    let password = match user {
        "kalle" => "kissa2",
        "uolevi" => "12345",
        "Olaf" => "ILoveSummer",
        _ => panic!(),
    };
    command()
        .args(&["login"])
        .write_stdin(format!("{}\n{}\n", user, password))
        .assert()
        .success();
}

pub fn create_file<P: AsRef<Path>, S: AsRef<[u8]>>(filename: P, content: S) {
    let mut file = std::fs::File::create(filename.as_ref()).unwrap();
    std::io::Write::write_all(&mut file, content.as_ref()).unwrap();
}
