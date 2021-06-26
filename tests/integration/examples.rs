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
