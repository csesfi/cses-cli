use anyhow::{anyhow, Result};

use crate::{Resources, Storage, RP};

// TODO: Unify with contest selection
pub fn select_course(res: &mut Resources<impl RP>, course_id: Option<String>) -> Result<String> {
    if let Some(course_id) = course_id {
        res.storage.get_mut().set_course(course_id);
        res.storage.save()?;
    }
    Ok(res
        .storage
        .get()
        .get_course()
        .ok_or_else(|| anyhow!("Course not provided"))?
        .to_owned())
}
