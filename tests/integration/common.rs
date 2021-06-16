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

fn read_file<P: AsRef<Path>>(filename: P, contents: &mut String) {
    let mut file = std::fs::File::open(filename.as_ref()).unwrap();
    std::io::Read::read_to_string(&mut file, contents).unwrap();
}

fn create_file<P: AsRef<Path>, S: AsRef<[u8]>>(filename: P, content: S) {
    let mut file = std::fs::File::create(filename.as_ref()).unwrap();
    std::io::Write::write_all(&mut file, content.as_ref()).unwrap();
}

pub fn load_file(filename: &str) {
    load_file_as(filename, filename);
}

pub fn load_file_as<P: AsRef<Path>>(filename: &str, new_filename: P) {
    let mut contents = String::new();
    read_file(format!("../tests/files/{}", filename), &mut contents);
    create_file(new_filename, contents);
}
