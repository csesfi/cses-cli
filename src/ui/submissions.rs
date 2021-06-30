use crate::entities::Scope;
use crate::service;
use crate::RP;
use anyhow::Result;
use std::io::Write;

use super::table::*;
use super::util::{format_code_size, format_code_time, styled_task_status_or_score};
use super::Ui;

pub fn list(ui: &mut Ui<impl RP>, scope: &Scope, task_id: &str) -> Result<()> {
    let submissions = service::submission_list(&mut ui.res, scope, task_id)?;
    if submissions.is_empty() {
        writeln!(ui.term, "No submissions yet!")?;
        return Ok(());
    }
    let mut table = Table::new(vec![0; 6]);
    table.add_row(vec![
        TableCell::from("ID").align(TableAlign::Center),
        TableCell::from("time").align(TableAlign::Center),
        TableCell::from("lang").align(TableAlign::Center),
        TableCell::from("code time")
            .align(TableAlign::Center)
            .allow_hiding(),
        TableCell::from("code size")
            .align(TableAlign::Center)
            .allow_hiding(),
        "result".into(),
    ]);
    table.add_separator();
    for submission in submissions {
        table.add_row(vec![
            submission.id.into(),
            submission.time.into(),
            TableCell::optional(submission.language.name),
            // TODO: hide this column in some cases? semantic difference between data not being in
            // JSON and it being null can't be distinguished with miniserde
            format_code_time(submission.code_time).into(),
            TableCell::optional(format_code_size(submission.size)),
            TableCell::styled(styled_task_status_or_score(
                submission.result,
                submission.score,
            ))
            .align(TableAlign::Center),
        ]);
    }
    write!(ui.term, "\n{}\n", table)?;
    Ok(())
}
