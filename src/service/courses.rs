use crate::{
    entities::{CourseContent, CourseInfo, Scope},
    CsesApi, Resources, Storage, RP,
};
use anyhow::Result;

pub fn courses(res: &mut Resources<impl RP>) -> Result<Vec<CourseInfo>> {
    let token = res.storage.get().get_token();
    let courses = res.api.get_courses(token)?.courses;
    Ok(courses)
}

pub fn course_content(res: &mut Resources<impl RP>, scope: &Scope) -> Result<CourseContent> {
    let token = res.storage.get().get_token();
    let course_content = res.api.get_content(token, scope)?;
    Ok(course_content)
}
