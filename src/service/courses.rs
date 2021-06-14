use crate::{CsesApi, RP, Resources, entities::CourseInfo};
use anyhow::Result;

pub fn courses(res: &mut Resources<impl RP>) -> Result<Vec<CourseInfo>> {
    let courses = res.api.get_courses()?.courses;
    Ok(courses)
}