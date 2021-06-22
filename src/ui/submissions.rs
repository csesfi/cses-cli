use crate::entities::Scope;
use crate::service;
use crate::RP;
use anyhow::Result;
use std::io::Write;

use super::courses::styled_task_status;
use super::table::*;
use super::util::format_code_time;
use super::Ui;

pub fn list(ui: &mut Ui<impl RP>, scope: &Scope, task_id: u64) -> Result<()> {
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
            TableCell::optional(submission.size),
            // FIXME: unwrap
            TableCell::styled(styled_task_status(submission.result)).align(TableAlign::Center),
        ]);
    }
    write!(ui.term, "\n{}\n", table)?;
    Ok(())
}
