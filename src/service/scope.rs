use anyhow::{anyhow, Result};

use crate::entities::Scope;
use crate::{Resources, Storage, RP};

pub fn select_scope(res: &mut Resources<impl RP>, changed_scope: Option<Scope>) -> Result<Scope> {
    if let Some(scope) = changed_scope {
        res.storage.get_mut().set_scope(scope);
        res.storage.save()?;
    }
    res.storage
        .get()
        .get_scope()
        .ok_or_else(|| anyhow!("Course or contest not provided"))
}
