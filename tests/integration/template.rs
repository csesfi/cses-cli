use crate::common::*;
#[distributed_slice(TESTS)]
fn download_template_fails_without_course() {
    log_in();
    command()
        .args(&["template"])
        .assert()
        .failure()
        .stdout(contains("not provided"));
}
#[distributed_slice(TESTS)]
fn download_template_succeeds_with_only_course() {
    log_in();
    command()
        .args(&["template", "-c", "cses"])
        .assert()
        .success()
        .stdout(regex_match(r"successfully.*code1"));
    let content = std::fs::read_to_string("./code1").unwrap();
    assert_eq!(content, "code1");
}
#[distributed_slice(TESTS)]
fn download_template_doesnt_overwrite_existing_file_if_user_says_no() {
    create_file("code1", "");
    log_in();
    command()
        .args(&["template", "-c", "cses"])
        .write_stdin("no")
        .assert()
        .success()
        .stdout(contains(r"code1 already exists"));
    let content = std::fs::read_to_string("./code1").unwrap();
    assert_eq!(content, "");
}
#[distributed_slice(TESTS)]
fn download_template_overwrites_existing_file_if_user_says_yes() {
    create_file("code1", "");
    log_in();
    command()
        .args(&["template", "-c", "cses"])
        .write_stdin("yes")
        .assert()
        .success()
        .stdout(contains(r"code1 already exists"));
    let content = std::fs::read_to_string("./code1").unwrap();
    assert_eq!(content, "code1");
}
#[distributed_slice(TESTS)]
fn download_template_succeeds_without_log_in() {
    command()
        .args(&["template", "-c", "cses", "-t", "1", "-l", "Rust"])
        .assert()
        .success()
        .stdout(contains(r"successfully"));
    let content = std::fs::read_to_string("./rust1.rs").unwrap();
    assert_eq!(content, "rust1");
}
#[distributed_slice(TESTS)]
fn download_template_succeeds_with_file_name() {
    command()
        .args(&["template", "-c", "cses", "-f", "rust3.rs"])
        .assert()
        .success()
        .stdout(contains(r"successfully"));
    let content = std::fs::read_to_string("./rust3.rs").unwrap();
    assert_eq!(content, "rust3");
}
#[distributed_slice(TESTS)]
fn non_existent_template_fails() {
    command()
        .args(&["template", "-c", "cses", "-f", "not_found.rs"])
        .assert()
        .failure()
        .stdout(contains(r"Failed querying code template"));
}

#[distributed_slice(TESTS)]
fn can_fetch_contest_template() {
    log_in();
    command()
        .args(&["template", "-c", "101"])
        .assert()
        .success()
        .stdout(regex_match(r"successfully.*code1"));
    let content = std::fs::read_to_string("./code1").unwrap();
    assert_eq!(content, "code1");
}

#[distributed_slice(TESTS)]
fn can_fetch_section_templates() {
    command()
        .args(&["template", "-c", "teku", "-s", "2", "-l", "Rust"])
        .assert()
        .success()
        .stdout(regex_match(r"Wierd algorithm.*rust1.rs"))
        .stdout(regex_match(r"Increasing array.*rust2.rs"));
    let content = std::fs::read_to_string("./rust1.rs").unwrap();
    assert_eq!(content, "rust1");
    let content = std::fs::read_to_string("./rust2.rs").unwrap();
    assert_eq!(content, "rust2");
}

#[distributed_slice(TESTS)]
fn can_fetch_all_tests() {
    command()
        .args(&["template", "-c", "teku", "--all", "-l", "Rust"])
        .assert()
        .success()
        .stdout(regex_match(r"Wierd algorithm.*rust1.rs"))
        .stdout(regex_match(r"Increasing array.*rust2.rs"));
    let content = std::fs::read_to_string("./rust1.rs").unwrap();
    assert_eq!(content, "rust1");
    let content = std::fs::read_to_string("./rust2.rs").unwrap();
    assert_eq!(content, "rust2");
}
