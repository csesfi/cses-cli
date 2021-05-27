use crate::common::*;

#[distributed_slice(TESTS)]
fn help_works() {
    let assert = command().args(&["help"]).assert();
    assert
        .success()
        .stdout(regex_match(r"(?i)usage"))
        .stderr(predicate::str::is_empty());
}
