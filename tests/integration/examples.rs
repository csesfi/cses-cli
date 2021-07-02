use std::path::Path;

use crate::common::*;

#[distributed_slice(TESTS)]
fn test_task_escaped_properly() {
    // if the task is escaped properly, it is not 404 so it exists
    command()
        .args(&[
            "examples",
            "-c",
            "teku",
            "--task",
            "404&extra_nonexistent_param=123",
        ])
        .assert()
        .success();
}

#[distributed_slice(TESTS)]
fn example_test_files_are_saved_to_program_root() {
    fetch_examples(".");
}

#[distributed_slice(TESTS)]
fn example_test_files_are_saved_to_the_given_path() {
    fetch_examples("tests/t1");
}

#[distributed_slice(TESTS)]
fn the_user_is_asked_before_the_files_are_overwritten_on_program_root() {
    fetch_examples(".");

    command()
        .args(&["examples", "-c", "teku", "-t", "1"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)1\.in[\s\S]+2\.out[\s\S]+3\.in"))
        .stdout(regex_match(r"(?i)yes/no"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn the_user_is_asked_before_the_files_are_overwritten_on_custom_path() {
    fetch_examples("tests/t1");

    command()
        .args(&["examples", "-c", "teku", "-t", "1", "./tests/t1"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)yes/no"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn the_files_are_not_overwritten_if_the_user_doesnt_wants_to() {
    create_file("1.in", b"Hello");
    create_file("1.out", b"olleH");

    command()
        .args(&["examples", "-c", "teku", "-t", "1"])
        .write_stdin("no\n")
        .assert()
        .success()
        .stdout(regex_match(r"(?i)1\.in[\s\S]+1\.out"))
        .stdout(regex_match(r"(?i)yes/no"))
        .stderr(predicate::str::is_empty());

    assert_eq!("Hello", std::fs::read_to_string("1.in").unwrap());
    assert_eq!("olleH", std::fs::read_to_string("1.out").unwrap());
    assert!(!Path::new("./2.in").exists())
}

#[distributed_slice(TESTS)]
fn the_files_are_overwritten_if_the_user_wants_to() {
    create_file("1.in", b"Hello");
    create_file("1.out", b"olleH");

    command()
        .args(&["examples", "-c", "teku", "-t", "1"])
        .write_stdin("yes\n")
        .assert()
        .success()
        .stdout(regex_match(r"(?i)1\.in[\s\S]+1\.out"))
        .stdout(regex_match(r"(?i)yes/no"))
        .stderr(predicate::str::is_empty());

    assert_ne!("Hello", std::fs::read_to_string("1.in").unwrap());
    assert_ne!("olleH", std::fs::read_to_string("1.out").unwrap());
    assert!(Path::new("./2.in").exists())
}

#[distributed_slice(TESTS)]
fn files_for_other_test_cases_than_1_are_also_detected() {
    create_file("3.out", b"Hello from the otter slide.");

    command()
        .args(&["examples", "-c", "teku", "-t", "1"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)3\.out"))
        .stdout(regex_match(r"(?i)yes/no"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn the_user_is_not_asked_for_files_that_will_not_be_overwritten() {
    // NOTE: the current implementation only checks 1.in and 1.out
    create_file("4.in", b"Hello");
    create_file("4.out", b"olleH");

    command()
        .args(&["examples", "-c", "teku", "-t", "1"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)4\.in|4\.out").not())
        .stdout(regex_match(r"(?i)yes/no").not())
        .stderr(predicate::str::is_empty());
}

fn fetch_examples(path: &str) {
    let args = match path {
        "." => vec!["examples", "-c", "teku", "-t", "1"],
        _ => vec!["examples", "-c", "teku", "-t", "1", path],
    };
    command()
        .args(args)
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
    check_examples_exist(path);
}

fn check_examples_exist(path: &str) {
    for i in 1..4 {
        assert!(Path::new(&format!("{}/{}.in", path, i)).exists());
        assert!(Path::new(&format!("{}/{}.out", path, i)).exists());
    }
}
