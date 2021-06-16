use crate::common::*;
use std::path::PathBuf;

#[distributed_slice(TESTS)]
fn fails_with_wrong_filename() {
    // This would of course work in production, but this asserts that the test server checks the
    // file name.
    log_in();
    load_file_as("main.cpp", "mian.cpp");

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
    log_in();
    let mut path = PathBuf::new();
    path.push("folder");
    std::fs::create_dir(&path).unwrap();
    path.push("main.rs");

    load_file_as("main.rs", &path);

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
    log_in();
    load_file("main.rs");

    command()
        .args(&["submit", "-c", "kurssi", "main.rs", "-t", "2", "-l", "Rust"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)status.*pending(.|\n)*status.*ready"));
}

#[distributed_slice(TESTS)]
fn shows_verdict() {
    log_in();
    load_file("main.rs");

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
    log_in();
    load_file("main.cpp");

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
fn remembers_course() {
    log_in();
    load_file("13.rs");

    command()
        .args(&[
            "submit", "-c", "cses", "-t", "13", "-l", "C++", "-o", "C++17", "13.rs",
        ])
        .assert()
        .success();

    load_file("main.cpp");
    command()
        .args(&["submit", "-t", "42", "-l", "C++", "-o", "C++17", "main.cpp"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)status.*ready"));
}
#[distributed_slice(TESTS)]
fn does_not_remember_language_or_option() {
    log_in();
    load_file("13.rs");

    command()
        .args(&[
            "submit", "-c", "cses", "-t", "13", "-l", "C++", "-o", "C++17", "13.rs",
        ])
        .assert()
        .success();

    command()
        .args(&["submit", "-c", "cses", "-t", "13", "-o", "C++17", "13.rs"])
        .assert()
        .failure()
        .stdout(contains(r"Failed submitting file"));

    command()
        .args(&[
            "submit", "-c", "cses", "-t", "13", "-l", "C++", "-o", "C++17", "13.rs",
        ])
        .assert()
        .success();

    command()
        .args(&["submit", "-c", "cses", "-t", "13", "-l", "C++", "13.rs"])
        .assert()
        .failure()
        .stdout(contains(r"Failed submitting file"));
}

#[distributed_slice(TESTS)]
fn compiler_report_is_dispayed_with_compile_error() {
    log_in();
    load_file("13.rs");

    let assert = command()
        .args(&[
            "submit", "13.rs", "-c", "cses", "-t", "13", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(contains(r"COMPILE ERROR"))
        .stdout(regex_match(r"(?i)compiler"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn compiler_report_is_not_displayed_without_any_content() {
    log_in();
    load_file("main.cpp");

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
    log_in();
    load_file("main.cpp");

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "cses", "-t", "42", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(contains(r"READY"))
        .stdout(regex_match(r"(?i)compiler"))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn sender_name_is_displayed() {
    log_in();
    load_file("main.cpp");

    command()
        .args(&[
            "submit", "main.cpp", "-c", "cses", "-t", "42", "-l", "C++", "-o", "C++17",
        ])
        .assert()
        .success()
        .stdout(regex_match("(?i)sender"))
        .stdout(contains("uolevi@cses.fi (mooc.fi)"));
}

#[distributed_slice(TESTS)]
fn null_test_time_finishes_and_is_printed_correctly() {
    log_in();
    load_file("main.cpp");

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "progress", "-t", "7", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(contains(r"--"))
        .stdout(contains(r"Result"))
        .stderr(predicate::str::is_empty());
}
#[distributed_slice(TESTS)]
fn null_test_time_finishes_and_is_print() {
    log_in();
    load_file("main.cpp");

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "progress", "-t", "7", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(contains(r"--"))
        .stdout(contains(r"Result"))
        .stderr(predicate::str::is_empty());
}
#[distributed_slice(TESTS)]
fn submission_works_without_language_and_option() {
    log_in();
    load_file("main.cpp");

    command()
        .args(&["submit", "-c", "cses", "-t", "444", "main.cpp"])
        .assert()
        .success()
        .stdout(contains(r"Result: WRONG ANSWER"))
        .stdout(contains(r"Language: C++ (C++17)"));
}
#[distributed_slice(TESTS)]
fn submission_works_without_language_with_option() {
    log_in();
    load_file("main.cpp");

    command()
        .args(&[
            "submit", "-c", "cses", "-t", "555", "-o", "C++17", "main.cpp",
        ])
        .assert()
        .success()
        .stdout(contains(r"Result: WRONG ANSWER"))
        .stdout(contains(r"Language: C++ (C++17)"));
}
#[distributed_slice(TESTS)]
fn test_server_returns_null_language() {
    log_in();
    load_file_as("main.cpp", "main.asdf");

    command()
        .args(&["submit", "-c", "cses", "-t", "111", "main.asdf"])
        .assert()
        .success()
        .stdout(contains(r"Result: INVALID LANGUAGE"))
        .stdout(contains(r"Language: ?"));
}
#[distributed_slice(TESTS)]
fn submission_works_without_task() {
    log_in();
    load_file("main.cpp");

    command()
        .args(&[
            "submit", "-c", "cses", "-l", "C++", "-o", "C++17", "main.cpp",
        ])
        .assert()
        .success()
        .stdout(contains(r"Result: ACCEPTED"))
        .stdout(contains(r"Language: C++ (C++17)"));
}
#[distributed_slice(TESTS)]
fn test_report_is_displayed_with_content() {
    log_in();
    load_file("lucky.py");

    let assert = command()
        .args(&[
            "submit", "lucky.py", "-c", "tira21k", "-t", "23", "-l", "CPython",
        ])
        .assert();
    assert
        .success()
        .stdout(regex_match(r"READY"))
        .stdout(regex_match(r"(?i)Test report"))
        .stderr(predicate::str::is_empty());
}
#[distributed_slice(TESTS)]
fn test_report_is_not_displayed_without_any_content() {
    log_in();
    load_file("main.cpp");

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "progress", "-t", "7", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(regex_match(r"READY"))
        .stdout(regex_match(r"(?i)Test report").not())
        .stderr(predicate::str::is_empty());
}
#[distributed_slice(TESTS)]
fn test_task_deduction_hint_printed() {
    log_in();
    load_file("main.cpp");

    let assert = command()
        .args(&[
            "submit",
            "main.cpp",
            "-c",
            "test_server_deduction",
            "-l",
            "C++",
            "-o",
            "C++17",
        ])
        .assert();
    assert
        .failure()
        .stdout(contains("cses-cli submit hello_world.rs -t 1337"));
}
#[distributed_slice(TESTS)]
fn test_language_deduction_hint_printed() {
    log_in();
    load_file_as("main.cpp", "main.ccp");

    let assert = command()
        .args(&[
            "submit",
            "main.ccp",
            "-c",
            "test_server_deduction",
            "-t",
            "1337",
        ])
        .assert();
    assert
        .failure()
        .stdout(contains("cses-cli submit hello_world.rs -l Rust"));
}
// At one point the openapi specification didn't
// allow server to return `client_error` to submission post.
#[distributed_slice(TESTS)]
fn test_client_error_doesnt_crash_server() {
    log_in();
    load_file("main.cpp");

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "progress", "-t", "123123", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert.failure().stdout(contains("miniserde error").not());
}

#[distributed_slice(TESTS)]
fn cannot_read_a_file_that_is_too_large() {
    log_in();
    load_file("big.file");

    command()
        .args(&["submit", "big.file", "-c", "alon"])
        .assert()
        .failure()
        .stdout(regex_match(r"(?i)too large"))
        .stdout(regex_match(r"(?i)limit"))
        .stderr(predicate::str::is_empty());
}
