use crate::common::*;

#[distributed_slice(TESTS)]
fn help_works() {
    let assert = command().args(&["help"]).assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)usage"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn h_flag_works() {
    let assert = command().args(&["-h"]).assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)usage"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn help_flag_works() {
    let assert = command().args(&["--help"]).assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)usage"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn h_flag_overrides_login_command() {
    let assert = command().args(&["login", "-h"]).assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)usage"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn help_flag_overrides_submit_command() {
    let assert = command().args(&["submit", "-h"]).assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)usage"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn hint_is_displayed_if_no_command_is_provided() {
    let assert = command().assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)usage"))
        .stdout(regex_match(r"(?i)login status"))
        .stderr(predicate::str::is_empty());
}
