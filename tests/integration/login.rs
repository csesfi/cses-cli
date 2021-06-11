use assert_cmd::assert::Assert;

use crate::common::*;

#[distributed_slice(TESTS)]
fn login_is_successful_with_correct_username_and_password() {
    let assert = successful_login_attempt();
    verify_successful_login_output(assert);
}

// FIXME: Add equivalent test for web-based auth
// #[distributed_slice(TESTS)]
// fn login_fails_with_incorrect_username() {
//     command()
//         .args(&["login"])
//         .write_stdin("nonexistinguser\nincorrectpassword\n")
//         .assert()
//         .failure();
// }

#[distributed_slice(TESTS)]
fn user_can_log_out() {
    let assert = successful_login_attempt();
    verify_successful_login_output(assert);
    let assert = logout_user();
    assert
        .success()
        .stdout(regex_match(r"(?i)login invalidated successfully"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn application_knows_user_is_already_logged_in() {
    let assert = successful_login_attempt();
    verify_successful_login_output(assert);
    let assert = command().args(&["login"]).write_stdin("no").assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)already logged in"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn user_can_overwrite_current_login() {
    let assert = successful_login_attempt();
    verify_successful_login_output(assert);
    let assert = command().args(&["login"]).write_stdin("yes\n").assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)overwrite"))
        .stdout(regex_match(r"(?i)please visit"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn test_login_returns_correct_link() {
    command()
        .args(&["login"])
        .assert()
        .success()
        .stdout(contains("http://127.0.0.1:4011/authorize-login?token="));
}

fn successful_login_attempt() -> Assert {
    command().args(&["login"]).assert()
}

fn verify_successful_login_output(assert: Assert) {
    assert
        .success()
        .stdout(regex_match(r"(?i)please visit"))
        .stderr(predicate::str::is_empty());
}

fn logout_user() -> Assert {
    command().args(&["logout"]).assert()
}
