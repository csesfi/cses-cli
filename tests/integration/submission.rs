use crate::common::*;

#[distributed_slice(TESTS)]
fn submission_list_contains_all_fields_with_content() {
    log_in();
    command()
        .args(&["submissions", "-c", "cses", "-t", "1"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)ID"))
        .stdout(regex_match(r"(?i)time"))
        .stdout(regex_match(r"(?i)lang"))
        .stdout(regex_match(r"(?i)code time"))
        .stdout(regex_match(r"(?i)code size"))
        .stdout(regex_match(r"(?i)result"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn submission_list_does_not_contain_fields_with_no_content() {
    log_in();
    command()
        .args(&["submissions", "-c", "cses", "-t", "2"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)ID"))
        .stdout(regex_match(r"(?i)time"))
        .stdout(regex_match(r"(?i)lang"))
        .stdout(regex_match(r"(?i)code time")) // Should be missing
        .stdout(regex_match(r"(?i)code size").not())
        .stdout(regex_match(r"(?i)result"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn if_submission_list_is_empty_show_message() {
    log_in();
    command()
        .args(&["submissions", "-c", "cses", "-t", "404"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)ID").not())
        .stdout(regex_match(r"(?i)time").not())
        .stdout(regex_match(r"(?i)lang").not())
        .stdout(regex_match(r"(?i)code time").not())
        .stdout(regex_match(r"(?i)code size").not())
        .stdout(regex_match(r"(?i)result").not())
        .stdout(regex_match(r"(?i)No submissions"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn old_submission_can_be_viewed() {
    log_in();
    command()
        .args(&["submission", "-c", "cses", "1"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)Result"))
        .stderr(predicate::str::is_empty());
}
