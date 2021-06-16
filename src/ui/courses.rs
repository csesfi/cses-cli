use crate::{RP, entities::CourseItem, service};
use anyhow::Result;
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
        writeln!(ui.term, "\n{}", section.header)?;
        if let Some(text) = &section.text {
            writeln!(ui.term, "\n{}", text)?;
        }
        for course_item_raw in section.list {
            let course_item_as_enum = course_item_raw.as_enum()?;
            match course_item_as_enum {
                CourseItem::Text {name, link, ..} => {
                    writeln!(ui.term, "\n{}\n{}", name, link)?;
                }
                CourseItem::Link { name, link } => {
                    writeln!(ui.term, "\n{}\n{}", name, link)?;
                },
                CourseItem::Task { name, id, link, status }  => {
                    writeln!(ui.term, "\nTask\n{}\n{}\n{}\nStatus: {}", name, id, link, status)?;
                },
            }
        }
    }
    Ok(())
}
