use crate::common::*;

const RS_13_CONTENT: &str = "use std::io;\n\nfn main() {\n";
const MAIN_CPP_CONTENT: &str = "#include <iostream>\n";

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
