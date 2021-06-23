use crate::{
    entities::{CourseItem, CourseItemRaw, CourseTaskStatus, Scope},
    service,
    ui::table::{Table, TableAlign, TableCell},
    RP,
};
use anyhow::Result;
use console::{style, StyledObject};
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
    let course = service::course_content(&mut ui.res, scope)?;

    if course.sections.is_empty() {
        return Ok(writeln!(ui.term, "No course content!")?);
    }

    for section in course.sections {
        writeln!(ui.term, "\n{}", style(&section.header).bold())?;
        if let Some(text) = &section.text {
            writeln!(ui.term, "\n{}", text)?;
        }

        let table = create_course_item_table(&section.list)?;
        write!(ui.term, "\n{}", table)?;
    }
    writeln!(ui.term)?;
    Ok(())
}

pub fn create_course_item_table(list: &[CourseItemRaw]) -> Result<Table> {
    let mut table = Table::new(vec![4, 30, 0, 0]);

    for course_item_raw in list {
        let course_item_as_enum = course_item_raw.as_enum()?;
        match course_item_as_enum {
            CourseItem::Text { id, name, link } => {
                table.add_row(vec![
                    TableCell::from(id).align(TableAlign::Right),
                    TableCell::from(name),
                    TableCell::empty(),
                    TableCell::from(link),
                ]);
            }
            CourseItem::Link { name, link } => {
                table.add_row(vec![
                    TableCell::from(""),
                    TableCell::from(name),
                    TableCell::empty(),
                    TableCell::from(link),
                ]);
            }
            CourseItem::Task {
                name,
                id,
                link,
                status,
                score,
            } => {
                table.add_row(vec![
                    TableCell::from(id).align(TableAlign::Right),
                    TableCell::from(name),
                    TableCell::styled(styled_task_status_or_score(status, score)),
                    TableCell::from(link),
                ]);
            }
        }
    }
    Ok(table)
}

pub fn styled_task_status_or_score(
    status: Option<CourseTaskStatus>,
    score: Option<u64>,
) -> StyledObject<String> {
    if let Some(points) = score {
        return match points {
            points if points >= 100 => style(points.to_string()).black().on_green(),
            points if points > 60 => style(points.to_string()).green(),
            points if points > 10 => style(points.to_string()).yellow(),
            _ => style(points.to_string()).red(),
        };
    }

    match status {
        Some(CourseTaskStatus::Pass) => style("+".to_string()).green(),
        Some(CourseTaskStatus::Fail) => style("X".to_string()).red(),
        Some(CourseTaskStatus::None) => style("-".to_string()).dim(),
        None => style("-".to_string()).dim(),
    }
}
