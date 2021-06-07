use crate::service;
use crate::RP;
use anyhow::Result;
use console::{Style, StyledObject};
use std::io::Write;

use super::Ui;
use crate::entities::SubmissionInfo;

pub fn print_submission_info(
    ui: &mut Ui<impl RP>,
    submission_id: u64,
    long_poll: bool,
) -> Result<()> {
    let mut submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
    print_info_header(ui, &submission_info)?;
    let mut compiler_report_printed = print_compiler_report(ui, &submission_info)?;
    print_status(ui, &submission_info)?;
    while submission_info.pending {
        submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
        ui.term.clear_line()?;
        if !compiler_report_printed {
            compiler_report_printed = print_compiler_report(ui, &submission_info)?;
        }
        print_status(ui, &submission_info)?;
    }
    write!(ui.term, "\n")?;
    print_test_results(ui, &submission_info)?;
    print_final_result(ui, &submission_info)?;
    Ok(())
}

fn print_info_header(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<()> {
    ui.term.write_line("Submission details\n")?;
    writeln!(ui.term, "Submission time: {}", submission_info.time)?;
    write!(ui.term, "Language: {}", submission_info.language.name)?;
    if let Some(ref option) = submission_info.language.option {
        write!(ui.term, " ({})", option)?;
    };
    Ok(writeln!(ui.term)?)
}

// Returns true if the compiler report was printed successfully
fn print_compiler_report(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<bool> {
    if let Some(compiler_report) = &submission_info.compiler {
        writeln!(ui.term, "\nCompiler report:\n{}", compiler_report)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn print_status(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<()> {
    let status_text = "Status:";
    if let Some(ref test_progress) = submission_info.test_progress {
        let progress_fraction =
            test_progress.finished_tests as f64 / test_progress.total_tests as f64;
        if (0.0..=1.0).contains(&progress_fraction) {
            let (_r, term_width) = ui.term.size();
            let mut text_width = console::measure_text_width(status_text) as u64;
            text_width += console::measure_text_width(&submission_info.status) as u64;
            text_width += 4;
            let progress_bar = progress_bar((term_width as u64) - text_width, progress_fraction)?;
            write!(
                ui.term,
                "{} {} {}",
                status_text, submission_info.status, progress_bar
            )?;
            return Ok(());
        }
    }
    write!(ui.term, "{} {}", status_text, submission_info.status)?;
    Ok(())
}
fn progress_bar(width: u64, progress_fraction: f64) -> Result<String> {
    let mut s = String::from("");
    let mut progress = progress_fraction * width as f64;
    while progress >= 1.0 {
        progress -= 1.0;
        s.push('#');
    }
    Ok(format!("[{:w$}]", s, w = width as usize))
}
fn print_test_results(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<()> {
    if let Some(ref tests) = submission_info.tests {
        ui.term.write_line("\nTest results\n")?;
        ui.term.write_line("  # |        verdict        | time")?;
        ui.term
            .write_line("-------------------------------------")?;
        for test in tests {
            write!(
                ui.term,
                "{:>3} | {:<21} | ",
                test.number,
                with_color(&test.verdict)
            )?;
            match test.time {
                Some(time) => writeln!(ui.term, "{:.2} s", time as f64 / 1000.0)?,
                None => ui.term.write_line("--")?,
            };
        }
    }
    Ok(())
}

fn print_final_result(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<()> {
    if let Some(ref result) = submission_info.result {
        writeln!(ui.term, "Result: {}", with_color(&result))?;
    };
    Ok(())
}

pub fn with_color(line: &str) -> StyledObject<&str> {
    let color;
    if line == "ACCEPTED" {
        color = Style::new().green();
    } else {
        color = Style::new().red();
    }
    color.apply_to(line)
}
