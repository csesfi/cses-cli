use std::io::Write;

use anyhow::Result;
use console::style;

use super::Ui;
use crate::entities::TaskStatement;
use crate::ui::util::format_code_time;
use crate::RP;

pub fn print_statement(ui: &mut Ui<impl RP>, task_statement: &TaskStatement) -> Result<()> {
    writeln!(ui.term, "\n{}", style(&task_statement.name).bold())?;
    if let Some(time_limit) = &task_statement.time_limit {
        writeln!(
            ui.term,
            "Time limit: {}",
            format_code_time(Some(time_limit.to_owned()))
        )?;
    }
    if let Some(memory_limit) = &task_statement.memory_limit {
        writeln!(ui.term, "Memory limit: {} MB", memory_limit)?;
    }
    writeln!(ui.term, "\n{}", style(&task_statement.text))?;
    writeln!(ui.term)?;

    Ok(())
}
