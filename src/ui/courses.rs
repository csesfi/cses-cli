use crate::{
    entities::{Scope, ScopeItem, ScopeItemRaw},
    service,
    ui::{
        table::{Table, TableAlign, TableCell},
        util::styled_task_status_or_score,
    },
    RP,
};
use anyhow::Result;
use console::style;
use std::io::Write;

use super::Ui;

pub fn list_courses(ui: &mut Ui<impl RP>) -> Result<()> {
    let courses = service::courses(&mut ui.res)?;

    if courses.is_empty() {
        return Ok(writeln!(ui.term, "No available courses!")?);
    }

    writeln!(ui.term, "Courses: ")?;
    for course in courses {
        writeln!(ui.term, "\n{} ({})", course.name, course.id)?;
        writeln!(ui.term, "  {}", course.description)?;
    }
    Ok(())
}

pub fn list_content(ui: &mut Ui<impl RP>, scope: &Scope) -> Result<()> {
    let scope_content = service::scope_content(&mut ui.res, scope)?;

    if scope_content.sections.is_empty() {
        return Ok(writeln!(
            ui.term,
            "{}",
            match scope {
                Scope::Course(_) => "No course content!",
                Scope::Contest(_) => "No contest content!",
            }
        )?);
    }

    for section in scope_content.sections {
        writeln!(ui.term, "\n{}", style(&section.header).bold())?;
        if let Some(text) = &section.text {
            writeln!(ui.term, "{}", text)?;
        }

        let table = create_item_table(&section.list)?;
        write!(ui.term, "{}", table)?;
    }
    writeln!(ui.term)?;
    Ok(())
}

pub fn create_item_table(list: &[ScopeItemRaw]) -> Result<Table> {
    let mut table = Table::new(vec![0, 0, 0, 0]);

    for scope_item_raw in list {
        let scope_item_as_enum = scope_item_raw.as_enum()?;
        match scope_item_as_enum {
            ScopeItem::Text { id, name, link } => {
                table.add_row(vec![
                    TableCell::from(id).align(TableAlign::Right),
                    TableCell::from(name),
                    TableCell::empty(),
                    TableCell::from(link),
                ]);
            }
            ScopeItem::Link { name, link } => {
                table.add_row(vec![
                    TableCell::empty(),
                    TableCell::from(name),
                    TableCell::empty(),
                    TableCell::from(link),
                ]);
            }
            ScopeItem::Task {
                name,
                id,
                link,
                status,
                score,
            } => {
                table.add_row(vec![
                    TableCell::from(id).align(TableAlign::Right),
                    TableCell::from(name),
                    TableCell::styled(styled_task_status_or_score(status, score))
                        .align(TableAlign::Center),
                    TableCell::from(link),
                ]);
            }
        }
    }
    Ok(table)
}
