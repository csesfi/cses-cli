use super::login_is_valid;
use crate::{CsesApi, RP, Resources, Storage, entities::{CourseContent, CourseInfo}};
use anyhow::Result;

pub fn courses(res: &mut Resources<impl RP>) -> Result<Vec<CourseInfo>> {
    let mut token = None;
    if login_is_valid(res)? {
        token = res.storage.get().get_token();
    }
    let courses = res.api.get_courses(token)?.courses;
    Ok(courses)
}

pub fn course_content(res: &mut Resources<impl RP>, course_id: &str) -> Result<CourseContent> {
    let mut token = None;
    if login_is_valid(res)? {
        token = res.storage.get().get_token();
    }
    let course_content = res.api.get_course_content(token, course_id)?;
    Ok(course_content)
}
