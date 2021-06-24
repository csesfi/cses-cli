use crate::common::*;

#[distributed_slice(TESTS)]
fn list_shows_task_char_and_points() {
    command()
        .args(&["list", "-c", "1"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)a.*summa.*100"));
}

#[distributed_slice(TESTS)]
fn list_works_with_long_contest_opt() {
    command()
        .args(&["list", "--contest", "1"])
        .assert()
        .success()
        .stdout(regex_match(r"(?i)tasks"));
}
