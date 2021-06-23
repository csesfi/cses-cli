use crate::{
    entities::{CourseInfo, Scope, ScopeContent},
    CsesApi, Resources, Storage, RP,
};
use anyhow::Result;

pub fn courses(res: &mut Resources<impl RP>) -> Result<Vec<CourseInfo>> {
    let token = res.storage.get().get_token();
    let courses = res.api.get_courses(token)?.courses;
    Ok(courses)
}

pub fn scope_content(res: &mut Resources<impl RP>, scope: &Scope) -> Result<ScopeContent> {
    let token = res.storage.get().get_token();
    let scope_content = res.api.get_content(token, scope)?;
    Ok(scope_content)
}
