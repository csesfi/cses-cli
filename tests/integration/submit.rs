use crate::common::*;

const TEST_CPP_CONTENT: &[u8] = b"asd
asd
asd";

#[distributed_slice(TESTS)]
fn compiler_report_is_dispayed_with_compile_error() {
    log_in();
    create_file(&"test.cpp", &TEST_CPP_CONTENT).unwrap();

    command()
        .args(&[
            "submit",
            "test.cpp",
            "--course-id",
            "comp",
            "--task-id",
            "1337",
        ])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)compiler"))
        .stderr(predicate::str::is_empty());
}

// TODO: Figure out how to test that something isn't in the stdout
// #[distributed_slice(TESTS)]
// fn compiler_report_is_not_displayed_without_any_content() {
//     log_in();
//     create_file(&"test.cpp",  &TEST_CPP_CONTENT).unwrap();

//     command()
//         .args(&["submit", "test.cpp", "--course-id", "comp", "--task-id", "1337"])
//         .assert()
//         .success()
//         .stdout(regex_match(r"(?i)compiler"))
//         .stderr(predicate::str::is_empty());
// }

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
