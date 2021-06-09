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
    let mut spinner = Spinner::new(9);
    while submission_info.pending {
        spinner.rotate_and_print(ui)?;
        submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
        ui.term.clear_last_lines(2)?;
        if !compiler_report_printed {
            compiler_report_printed = print_compiler_report(ui, &submission_info)?;
        }
        print_status(ui, &submission_info)?;
    }
    writeln!(ui.term)?;
    print_test_report(ui, &submission_info)?;
    print_test_results(ui, &submission_info)?;
    print_final_result(ui, &submission_info)?;
    Ok(())
}

fn print_info_header(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<()> {
    ui.term.write_line("Submission details\n")?;
    writeln!(ui.term, "Submission time: {}", submission_info.time)?;
    write!(
        ui.term,
        "Language: {}",
        submission_info.language.name.as_deref().unwrap_or("?")
    )?;
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
            let mut text_width = status_text.chars().count() as u64;
            text_width += submission_info.status.chars().count() as u64;
            text_width += 4;
            let bar_width = (term_width as u64).saturating_sub(text_width);
            let progress_bar = progress_bar(bar_width, progress_fraction)?;
            writeln!(
                ui.term,
                "{} {} {}",
                status_text, submission_info.status, progress_bar
            )?;
            return Ok(());
        }
    }
    writeln!(ui.term, "{} {}", status_text, submission_info.status)?;
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

fn print_test_report(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<()> {
    if let Some(ref report) = submission_info.test_report {
        ui.term.write_line("\nTest report:")?;
        writeln!(ui.term, "{}", report)?;
    };
    Ok(())
}

fn print_final_result(ui: &mut Ui<impl RP>, submission_info: &SubmissionInfo) -> Result<()> {
    if let Some(ref result) = submission_info.result {
        writeln!(ui.term, "\nResult: {}", with_color(&result))?;
    };
    Ok(())
}

fn with_color(line: &str) -> StyledObject<&str> {
    let color = match line {
        "ACCEPTED" => Style::new().green(),
        "UNKNOWN" => Style::new().white(),
        _ => Style::new().red(),
    };
    color.apply_to(line)
}

pub struct Spinner {
    state: String,
    width: usize,
}
impl Spinner {
    pub fn new(width: usize) -> Spinner {
        Spinner {
            state: String::from(""),
            width,
        }
    }
    fn rotate(&mut self) {
        match self.state.as_str() {
            "|" => self.state = String::from("/"),
            "/" => self.state = String::from("-"),
            "-" => self.state = String::from("\\"),
            _ => self.state = String::from("|"),
        }
    }
    fn print(&self, ui: &mut Ui<impl RP>) -> Result<()> {
        writeln!(
            ui.term,
            "{:>w$}",
            Style::new().bold().apply_to(&self.state),
            w = self.width
        )?;
        Ok(())
    }
    pub fn rotate_and_print(&mut self, ui: &mut Ui<impl RP>) -> Result<()> {
        self.rotate();
        self.print(ui)
    }
}
