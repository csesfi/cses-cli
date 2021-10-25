use std::path::Path;

use anyhow::{Context, Result};

use crate::entities::{Scope, TemplateResponse};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};

pub fn get_template(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    task_id: Option<&str>,
    language: Option<&str>,
    filename: Option<&str>,
) -> Result<TemplateResponse> {
    (|| -> Result<_> {
        let token = res.storage.get().get_token();
        Ok(res
            .api
            .get_template(token, scope, task_id, language, filename)?)
    })()
    .context("Failed querying code template from the server")
}

pub fn file_exists(res: &Resources<impl RP>, path: &Path) -> bool {
    res.filesystem.file_exists(path)
}

pub fn save_response(res: &mut Resources<impl RP>, response: &TemplateResponse) -> Result<()> {
    res.filesystem.write_file(
        &res.filesystem.decode_base64(&response.template_source)?,
        Path::new(&response.filename),
    )
}
