use crate::common::*;

const RS_13_CONTENT: &[u8] = b"use std::io;\n\nfn main() {\n";
const MAIN_CPP_CONTENT: &[u8] = b"#include <iostream>\n";

#[distributed_slice(TESTS)]
fn compiler_report_is_dispayed_with_compile_error() {
    log_in();
    create_file(&"13.rs", &RS_13_CONTENT).unwrap();

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
    log_in();
    create_file(&"main.cpp", &MAIN_CPP_CONTENT).unwrap();

    let assert = command()
        .args(&[
            "submit", "main.cpp", "-c", "alon", "-t", "4", "-l", "C++", "-o", "C++17",
        ])
        .assert();
    assert
        .success()
        .stdout(predicate::function(|string: &str| {
            !string.contains("Compiler")
        }))
        .stderr(predicate::str::is_empty());
}

#[distributed_slice(TESTS)]
fn compiler_report_is_dispayed_with_compiler_warnings() {
    log_in();
    create_file(&"main.cpp", &MAIN_CPP_CONTENT).unwrap();

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

fn create_file(filename: &str, content: &[u8]) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(&filename)?;
    std::io::Write::write_all(&mut file, &content)?;
    Ok(())
}

fn log_in() -> assert_cmd::assert::Assert {
    command()
        .args(["login"])
        .write_stdin("Olaf\nILoveSummer\n")
        .assert()
}
