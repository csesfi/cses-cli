use crate::service;
use crate::RP;
use anyhow::Result;
use console::{Style, StyledObject};

use super::Ui;
//use crate::entities::{Language, SubmissionInfo, SubmissionTestInfo};

pub fn print_submission_info(
    ui: &mut Ui<impl RP>,
    submission_id: u64,
    long_poll: bool,
) -> Result<()> {
    let mut submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
    /*
    let task = String::from("Weird Algorithm");
    let sender = String::from("username");
    let time = String::from("2021-05-31 10:32:41");
    let language = Language {
        name: String::from("C++"),
        option: Some(String::from("11")),
    };
    let status = String::from("PENDING");
    let pending = false;
    let result = Some(String::from("TIME LIMIT EXCEEDED"));
    let test1 = SubmissionTestInfo {
        number: 9,
        verdict: String::from("ACCEPTED"),
        time: Some(1),
    };
    let test2 = SubmissionTestInfo {
        number: 99,
        verdict: String::from("WRONG ANSWER"),
        time: Some(1),
    };
    let test3 = SubmissionTestInfo {
        number: 999,
        verdict: String::from("TIME LIMIT EXCEEDED"),
        time: None,
    };
    let mut tests = Vec::new();
    tests.push(test1);
    tests.push(test2);
    tests.push(test3);
    let tests = Some(tests);
    let mut submission_info = SubmissionInfo {
        task,
        sender,
        time,
        language,
        status,
        pending,
        result,
        tests,
    };
    */

    ui.term.write_line("Submission details\n")?;
    ui.term
        .write_line(&format!("Task: {}", submission_info.task))?;
    ui.term
        .write_line(&format!("Sender: {}", submission_info.sender))?;
    ui.term
        .write_line(&format!("Submission time: {}", submission_info.time))?;
    ui.term
        .write_str(&format!("Language: {}", submission_info.language.name))?;
    if let Some(option) = submission_info.language.option {
        ui.term.write_str(&option)?;
    };
    ui.term.write_line("")?;
    ui.term
        .write_line(&format!("Status: {}", submission_info.status))?;
    while submission_info.pending {
        submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
        ui.term.clear_last_lines(1)?;
        ui.term
            .write_line(&format!("Status: {}", submission_info.status))?;
    }

    if let Some(compiler_report) = &submission_info.compiler {
        ui.term
            .write_line(&format!("\nCompiler report:\n{}", compiler_report))?;
    }

    if let Some(result) = submission_info.result {
        ui.term
            .write_line(&format!("Result: {}", with_color(result)))?;
    };
    if let Some(tests) = submission_info.tests {
        ui.term.write_line("Test results\n")?;
        ui.term.write_line("test |       verdict        | time")?;
        ui.term.write_line("------------------------------------")?;
        for test in tests {
            ui.term.write_str(&format!(
                "#{:<3} | {:<20} | ",
                test.number,
                with_color(test.verdict)
            ))?;
            match test.time {
                Some(time) => ui
                    .term
                    .write_line(&format!("{:.2} s", time as f64 / 1000.0))?,
                None => ui.term.write_line("--")?,
            };
        }
    }
    Ok(())
}

pub fn with_color(line: String) -> StyledObject<String> {
    let mut color = Style::new().red();
    if line == "ACCEPTED" {
        color = Style::new().green();
    }
    color.apply_to(line)
}
