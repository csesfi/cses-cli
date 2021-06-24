use crate::entities::TaskStatement;
use crate::RP;
use anyhow::Result;
use console::style;
use std::io::Write;

use super::Ui;

pub fn print_statement(ui: &mut Ui<impl RP>, task_statement: &TaskStatement) -> Result<()> {
    writeln!(ui.term, "\n{}", style(&task_statement.name).bold())?;
    if let Some(time_limit) = &task_statement.time_limit {
        writeln!(ui.term, "\n{}", time_limit)?;
    }
    if let Some(memory_limit) = &task_statement.memory_limit {
        writeln!(ui.term, "\n{}", memory_limit)?;
    }
    writeln!(ui.term, "\n{}", style(&task_statement.text))?;
    writeln!(ui.term)?;
    
    Ok(())
}