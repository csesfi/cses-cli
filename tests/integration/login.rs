use assert_cmd::assert::Assert;

use crate::common::*;

#[distributed_slice(TESTS)]
fn login_is_successful_with_correct_username_and_password() {
    let assert = successful_login_attempt();
    assert.success()
        .stdout(regex_match(r"(?i)username: "))
        .stdout(regex_match(r"(?i)password: "))
        .stdout(regex_match(r"(?i)Login successful"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn login_fails_with_incorrect_username() {
    let assert = unsuccessful_login();
    assert.failure();
}

#[distributed_slice(TESTS)]
fn user_can_log_out() {
    let assert = successful_login_attempt();
    assert.success()
        .stdout(regex_match(r"(?i)username: "))
        .stdout(regex_match(r"(?i)password: "))
        .stdout(regex_match(r"(?i)Login successful"));
    let assert = logout_user();
    assert.success()
        .stdout(regex_match(r"(?i)Login invalidated successfully"))
        .stderr(predicate::str::is_empty());
}

fn successful_login_attempt() -> Assert {
    command()
        .args(&["login"])
        .write_stdin("kalle\nkissa2\n")
        .assert()
}

fn unsuccessful_login() -> Assert {
    command()
        .args(&["login"])
        .write_stdin("nonexistinguser\nincorrectpassword\n")
        .assert()
}

fn logout_user() -> Assert {
    command()
        .args(&["logout"])
        .assert()
}
