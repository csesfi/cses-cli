use crate::entities::SubmissionInfo;
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

    ui.term
        .write_line(&format!("Submission time: {}", submission_info.time))?;
    if let Some(option) = &submission_info.language.option {
        ui.term.write_line(&format!(
            "Language: {} {}",
            submission_info.language.name, option
        ))?
    } else {
        ui.term
            .write_line(&format!("Language: {}", submission_info.language.name))?
    }

    ui.term
        .write_line(&format!("Status: {}", submission_info.status))?;
    while submission_info.pending {
        submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
        ui.term.clear_last_lines(1)?;
        ui.term
            .write_line(&format!("Status: {}", submission_info.status))?;
    }

    if submission_info.status == "READY" {
        print_ready(ui, &submission_info)?
    }

    if let Some(compiler_report) = &submission_info.compiler {
        ui.term
            .write_str(&format!("\nCompiler report:\n{}", compiler_report))?;
    }

    Ok(())
}

fn print_ready(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<()> {
    if let Some(result) = &submission_info.result {
        ui.term
            .write_line(&format!("Result: {}", with_color(String::from(result))))?;
    }
    ui.term.write_line("# | verdict | time (ms)")?;
    ui.term.write_line("---------------------------")?;

    if let Some(tests) = &submission_info.tests {
        for test in tests {
            ui.term.write_line(&format!(
                "{} | {} | {}",
                test.number,
                with_color(String::from(&test.verdict)),
                test.time
            ))?;
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
