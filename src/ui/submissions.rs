use crate::service;
use crate::RP;
use anyhow::Result;
use std::io::Write;

use super::courses::styled_task_status;
use super::table::*;
use super::Ui;

pub fn list(ui: &mut Ui<impl RP>, task_id: u64) -> Result<()> {
    let submissions = service::submission_list(&mut ui.res, task_id)?.submissions;
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
        //TableCell::empty(),
    ]);
    table.add_separator();
    for submission in submissions {
        table.add_row(vec![
            submission.id.into(),
            submission.time.into(),
            TableCell::optional(submission.language.name),
            TableCell::optional(submission.code_time),
            TableCell::optional(submission.size),
            TableCell::styled(styled_task_status(submission.result)).align(TableAlign::Center),
        ]);
    }
    write!(ui.term, "{}", table)?;
    Ok(())
}
