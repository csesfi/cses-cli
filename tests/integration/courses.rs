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

#[distributed_slice(TESTS)]
fn all_course_section_headers_are_displayed() {
    command()
        .args(&["list", "-c", "teku"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)info"))
        .stdout(regex_match(r"(?i)week 1"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn course_section_optional_text_is_displayed() {
    command()
        .args(&["list", "--course", "teku"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)general info"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn course_item_parameters_are_displayed() {
    command()
        .args(&["list", "-c", "teku"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)1068"))
        .stdout(regex_match(r"(?i)algorithm"))
        .stdout(regex_match(r"(?i)https"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn task_status_is_none_when_not_logged_in() {
    command()
        .args(&["list", "-c", "teku"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)-"))
        .stdout(regex_match(r"(?i)(pass|fail)").not())
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn task_status_is_shown_when_logged_in() {
    log_in();
    command()
        .args(&["list", "-c", "teku"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)(pass|fail)"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn invalid_course_id_returns_error() {
    command()
        .args(&["list", "-c", "kute"])
        .assert()
        .failure()
        .stdout(contains("not found"))
        .stderr(predicate::str::is_empty());
}
