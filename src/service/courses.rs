use super::login_is_valid;
use crate::{entities::CourseInfo, CsesApi, Resources, Storage, RP};
use anyhow::Result;

pub fn courses(res: &mut Resources<impl RP>) -> Result<Vec<CourseInfo>> {
    let mut token = None;
    if login_is_valid(res)? {
        token = res.storage.get().get_token();
    }
    let courses = res.api.get_courses(token)?.courses;
    Ok(courses)
}
