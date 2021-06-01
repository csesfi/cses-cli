use crate::service;
use crate::RP;
use anyhow::Result;
use console::{Style, StyledObject};

use super::Ui;
use crate::entities::{Language, SubmissionInfo, SubmissionTestInfo};

pub fn print_submission_info(
    ui: &mut Ui<impl RP>,
    submission_id: u64,
    long_poll: bool,
) -> Result<()> {
    //let mut submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;

    let time = String::from("2021-05-31 10:32:41");
    let language = Language {
        name: String::from("RUST"),
        option: Some(String::from("c++17")),
    };
    let status = String::from("PENDING");
    let pending = true;
    let result = Some(String::from("ACCEPTED"));
    let test1 = SubmissionTestInfo {
        number: 1,
        verdict: String::from("ACCEPTED"),
        time: 1,
    };
    let test2 = SubmissionTestInfo {
        number: 2,
        verdict: String::from("WRONG ANSWER"),
        time: 2,
    };
    let mut tests = Vec::new();
    tests.push(test1);
    tests.push(test2);
    let tests = Some(tests);
    let mut submission_info = SubmissionInfo {
        time,
        language,
        status,
        pending,
        result,
        tests,
    };
    ui.term
        .write_line(&format!("Submission time: {}", submission_info.time))?;
    match submission_info.language.option {
        Some(option) => ui.term.write_line(&format!(
            "Language: {} {}",
            submission_info.language.name, option
        ))?,
        None => ui
            .term
            .write_line(&format!("Language: {}", submission_info.language.name))?,
    };
    ui.term
        .write_line(&format!("Status: {}", submission_info.status))?;
    while submission_info.pending {
        //submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
        submission_info.pending = false;
        submission_info.status = String::from("READY");
        ui.term.clear_last_lines(1)?;
        ui.term
            .write_line(&format!("Status: {}", submission_info.status))?;
    }
    if submission_info.status == "READY" {
        ui.term.write_line(&format!(
            "Result: {}",
            with_color(submission_info.result.unwrap())
        ))?;
        ui.term.write_line("# | verdict | time (ms)")?;
        ui.term.write_line("---------------------------")?;
        for test in submission_info.tests.unwrap() {
            ui.term.write_line(&format!(
                "{} | {} | {}",
                test.number,
                with_color(test.verdict),
                test.time
            ))?;
        }
    };
    Ok(())
}

pub fn with_color(line: String) -> StyledObject<String> {
    let mut color = Style::new().red();
    if line == "ACCEPTED" {
        color = Style::new().green();
    }
    color.apply_to(line)
}
