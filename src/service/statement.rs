use anyhow::{Context, Result};

use crate::entities::{Scope, TaskStatement};
use crate::{CsesApi, Resources, Storage, RP};
pub fn get_task_statement(
    res: &Resources<impl RP>,
    scope: &Scope,
    task_id: &str,
) -> Result<TaskStatement> {
    (|| -> Result<_> {
        let token = res.storage.get().get_token();
        Ok(res.api.get_task_statement(token, scope, task_id)?)
    })()
    .context("Failed querying task statement from the server")
}
