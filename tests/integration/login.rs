use crate::common::*;

#[distributed_slice(TESTS)]
fn login_asks_for_username() {
    let assert = command()
        .args(&["login"])
        .write_stdin("kalle\nkissa2\n")
        .assert();
    assert.success();
    // .stdout(regex_match(r"(?i)username: "))
    // .stderr(predicate::str::is_empty());
}
