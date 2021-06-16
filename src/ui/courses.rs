use crate::{RP, entities::{CourseItem, CourseTaskStatus}, service, ui::table::{Table, TableAlign, TableCell}};
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

pub fn list_course_content(ui: &mut Ui<impl RP>, course_id: &str) -> Result<()> {
    let course = service::course_content(&mut ui.res, course_id)?;

    if course.sections.is_empty() {
        return Ok(writeln!(ui.term, "No course content!")?);
    }
    
    for section in course.sections {
        writeln!(ui.term, "\n{}", style(&section.header).bold())?;
        if let Some(text) = &section.text {
            writeln!(ui.term, "\n{}", text)?;
        }

        let mut table = Table::new(vec![4, 30, 0, 0]);

        for course_item_raw in section.list {
            let course_item_as_enum = course_item_raw.as_enum()?;
            match course_item_as_enum {
                CourseItem::Text {id, name, link} => {
                    // writeln!(ui.term, "\n{}\n{}", name, link)?;
                    table.add_row(vec![
                        TableCell::from(id).align(TableAlign::Right),
                        TableCell::from(name),
                        TableCell::empty(),
                        TableCell::from(link),
                    ]);
                }
                CourseItem::Link { name, link } => {
                    table.add_row(vec![
                        TableCell::empty(),
                        TableCell::from(name),
                        TableCell::empty(),
                        TableCell::from(link),
                    ]);
                },
                CourseItem::Task { name, id, link, status }  => {
                    table.add_row(vec![
                        TableCell::from(id).align(TableAlign::Right),
                        TableCell::from(name),
                        TableCell::styled(match status {
                            CourseTaskStatus::Pass => style("+").green(),
                            CourseTaskStatus::Fail => style("X").red(),
                            CourseTaskStatus::None => style("-").dim(),
                        }),
                        TableCell::from(link),
                    ]);
                },
            }
        }
        write!(ui.term, "\n{}", table)?;
    }
    writeln!(ui.term)?;
    Ok(())
}
