use crate::common::*;
use std::path::PathBuf;

const MAIN_RS_CONTENT: &str = "use std::io;\n";
const RS_13_CONTENT: &str = "use std::io;\n\nfn main() {\n";
const MAIN_CPP_CONTENT: &str = "#include <iostream>\n";

#[distributed_slice(TESTS)]
fn fails_with_wrong_filename() {
    // This would of course work in production, but this asserts that the test server checks the
    // file name.
    log_in("kalle");
    create_file("mian.rs", MAIN_RS_CONTENT);

    command()
        .args(&[
            "submit",
            "--course",
            "kurssi",
            "mian.rs",
            "-t",
            "2",
            "--language",
            "Rust",
        ])
        .assert()
        .failure();
}

#[distributed_slice(TESTS)]
fn succeeds_with_correct_filename_in_folder() {
    log_in("kalle");
    let mut path = PathBuf::new();
    path.push("folder");
    std::fs::create_dir(&path).unwrap();
    path.push("main.rs");
    create_file(&path, MAIN_RS_CONTENT);

    command()
        .args(&[
            "submit",
            "--course",
            "kurssi",
            &path.display().to_string(),
            "-t",
            "2",
            "--language",
            "Rust",
        ])
        .assert()
        .success();
}

#[distributed_slice(TESTS)]
fn shows_status_pending_then_ready() {
    log_in("kalle");
    create_file("main.rs", MAIN_RS_CONTENT);

    command()
        .args(&["submit", "-c", "kurssi", "main.rs", "-t", "2", "-l", "Rust"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)status.*pending(.|\n)*status.*ready"));
}

#[distributed_slice(TESTS)]
fn shows_verdict() {
    log_in("kalle");
    create_file("main.rs", MAIN_RS_CONTENT);

    command()
        .args(&[
            "submit",
            "--course",
            "kurssi",
            "main.rs",
            "-t",
            "2",
            "--language",
            "Rust",
        ])
        .assert()
        .success()
        .stdout(regex_match("(?i)result.*accepted"));
}

#[distributed_slice(TESTS)]
fn shows_each_test_result() {
    log_in("uolevi");
    create_file("main.cpp", MAIN_CPP_CONTENT);

    command()
        .args(&[
            "submit", "-l", "C++", "--task", "4", "-c", "alon", "main.cpp", "-o", "C++17",
        ])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)1.*accepted"))
        .stdout(regex_match(r"(?i)2.*wrong answer"));
}

#[distributed_slice(TESTS)]
fn remembers_course_and_language() {
    log_in("uolevi");

    create_file("13.rs", RS_13_CONTENT);
    command()
        .args(&[
            "submit", "-c", "cses", "-t", "13", "-l", "C++", "-o", "C++17", "13.rs",
        ])
        .assert()
        .success();

    create_file("main.cpp", MAIN_CPP_CONTENT);
    command()
        .args(&["submit", "-t", "42", "main.cpp"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)status.*ready"));
}

#[distributed_slice(TESTS)]
fn compiler_report_is_dispayed_with_compile_error() {
    log_in("Olaf");
    create_file("13.rs", RS_13_CONTENT);

    let assert = command()
        .args(&[
            "submit", "13.rs", "-c", "cses", "-t", "13", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(regex_match(r"COMPILE ERROR"))
        .stdout(regex_match(r"(?i)compiler"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn compiler_report_is_not_displayed_without_any_content() {
    log_in("Olaf");
    create_file("main.cpp", MAIN_CPP_CONTENT);

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "alon", "-t", "4", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)compiler").not())
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn compiler_report_is_dispayed_with_compiler_warnings() {
    log_in("Olaf");
    create_file("main.cpp", MAIN_CPP_CONTENT);

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "cses", "-t", "42", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(regex_match(r"READY"))
        .stdout(regex_match(r"(?i)compiler"))
        .stderr(predicate::str::is_empty());
}
#[distributed_slice(TESTS)]
fn null_test_time_finishes_and_is_printed_correctly() {
    log_in("kalle");
    create_file("main.cpp", MAIN_CPP_CONTENT);

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "progress", "-t", "7", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(regex_match(r"--"))
        .stdout(regex_match(r"Result"))
        .stderr(predicate::str::is_empty());
}
