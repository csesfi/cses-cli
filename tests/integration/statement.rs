use crate::common::*;
#[distributed_slice(TESTS)]
fn test_logged_in_view_contest_short_flags() {
    log_in();
    command()
        .args(&["view", "-c", "123", "-t", "B"])
        .assert()
        .success()
        .stdout(contains("Test task"));
}
#[distributed_slice(TESTS)]
fn test_logged_out_view_contest_short_flags() {
    command()
        .args(&["view", "-c", "123", "-t", "B"])
        .assert()
        .success()
        .stdout(contains("Test task"));
}
#[distributed_slice(TESTS)]
fn test_view_course_short_flags() {
    command()
        .args(&["view", "-c", "teku", "-t", "34"])
        .assert()
        .success()
        .stdout(contains("Test task"));
}
#[distributed_slice(TESTS)]
fn test_view_contest_long_flags() {
    command()
        .args(&["view", "--contest", "123", "--task", "B"])
        .assert()
        .success()
        .stdout(contains("Test task"));
}
#[distributed_slice(TESTS)]
fn test_view_course_long_flags() {
    command()
        .args(&["view", "--course", "teku", "--task", "34"])
        .assert()
        .success()
        .stdout(contains("Test task"));
}
#[distributed_slice(TESTS)]
fn test_view_remembers_course() {
    command()
        .args(&["view", "-c", "123", "--task", "B"])
        .assert()
        .success()
        .stdout(contains("Test task"));
    command()
        .args(&["view", "--task", "B"])
        .assert()
        .success()
        .stdout(contains("Test task"));
}
#[distributed_slice(TESTS)]
fn test_api_error_printed() {
    command()
        .args(&["view", "-c", "nonexistent_course", "--task", "B"])
        .assert()
        .failure()
        .stdout(contains("Course not found"));
}
#[distributed_slice(TESTS)]
fn test_task_service_error_context_printed() {
    command()
        .args(&["view", "-c", "teku", "--task", "123123"])
        .assert()
        .failure()
        .stdout(regex_match(
            "(?i)failed querying task statement from the server",
        ));
}
#[distributed_slice(TESTS)]
fn test_task_with_time_limit_and_memory_limit_printed_correctly() {
    command()
        .args(&["view", "-c", "123", "--task", "B"])
        .assert()
        .success()
        .stdout(regex_match("Test task"))
        .stdout(regex_match("(?i)time limit.*1.00 s"))
        .stdout(regex_match("(?i)memory limit.*512 MB"))
        .stdout(contains("Solve this problem."));
}
#[distributed_slice(TESTS)]
fn test_task_without_time_limit_and_memory_limit_printed_correctly() {
    command()
        .args(&["view", "-c", "teku", "--task", "34"])
        .assert()
        .success()
        .stdout(contains("Test task"))
        .stdout(regex_match("(?i)time limit").not())
        .stdout(regex_match("(?i)memory limit").not())
        .stdout(contains("Solve this problem."));
}
#[distributed_slice(TESTS)]
fn test_task_escaped_properly() {
    command()
        .args(&[
            "view",
            "-c",
            "teku",
            "--task",
            "34&extra_nonexistent_param=123",
        ])
        .assert()
        .failure()
        .stdout(contains("Task not found"));
}
