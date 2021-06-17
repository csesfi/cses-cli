use crate::common::*;

#[distributed_slice(TESTS)]
fn can_get_a_list_of_courses_when_not_logged_in() {
    command()
        .args(&["courses"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)test course"))
        .stdout(regex_match(r"(?i)teku"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn can_get_a_list_of_courses_when_logged_in() {
    log_in();
    command()
        .args(&["courses"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)test course"))
        .stdout(regex_match(r"(?i)teku"))
        .stderr(predicate::str::is_empty());
}

// More to test the test server than the actual program
#[distributed_slice(TESTS)]
fn cannot_see_the_hidden_course_when_not_logged_in() {
    command()
        .args(&["courses"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)hidden course").not())
        .stderr(predicate::str::is_empty());
}

// More to test the test server than the actual program
#[distributed_slice(TESTS)]
fn can_see_the_hidden_course_when_logged_in() {
    log_in();
    command()
        .args(&["courses"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)hidden course"))
        .stderr(predicate::str::is_empty());
}
