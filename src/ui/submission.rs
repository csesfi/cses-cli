use crate::service;
use crate::RP;
use anyhow::Result;
use console::{Style, StyledObject};
use std::io::Write;

use super::Ui;
//use crate::entities::{Language, SubmissionInfo, SubmissionTestInfo};

pub fn print_submission_info(
    ui: &mut Ui<impl RP>,
    submission_id: u64,
    long_poll: bool,
) -> Result<()> {
    let mut submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
    ui.term.write_line("Submission details\n")?;
    writeln!(ui.term, "Submission time: {}", submission_info.time)?;
    write!(ui.term, "Language: {}", submission_info.language.name)?;
    if let Some(option) = submission_info.language.option {
        write!(ui.term, " ({})", option)?;
    };
    writeln!(ui.term)?;
    writeln!(ui.term, "Status: {}", submission_info.status)?;
    while submission_info.pending {
        submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
        ui.term.clear_last_lines(1)?;
        writeln!(ui.term, "Status: {}", submission_info.status)?;
    }

    if let Some(compiler_report) = &submission_info.compiler {
        writeln!(ui.term, "\nCompiler report:\n{}", compiler_report)?;
    }

    if let Some(result) = submission_info.result {
        writeln!(ui.term, "Result: {}", with_color(result))?;
    };
    if let Some(tests) = submission_info.tests {
        ui.term.write_line("\nTest results\n")?;
        ui.term.write_line("  # |        verdict        | time")?;
        ui.term
            .write_line("-------------------------------------")?;
        for test in tests {
            write!(
                ui.term,
                "{:>3} | {:<21} | ",
                test.number,
                with_color(test.verdict)
            )?;
            match test.time {
                Some(time) => writeln!(ui.term, "{:.2} s", time as f64 / 1000.0)?,
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