use crate::command;
use crate::entities::{Scope, TemplateResponse};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{Context, Result};

pub fn get_template(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    parameters: command::Template,
) -> Result<TemplateResponse> {
    (|| -> Result<_> {
        let token = res.storage.get().get_token();
        Ok(res.api.get_template(
            token,
            scope,
            parameters.task.as_deref(),
            parameters.language.as_deref(),
            parameters.filename.as_deref(),
        )?)
    })()
    .context("Failed querying code template from the server")
}

pub fn file_exists(res: &Resources<impl RP>, filename: &str) -> bool {
    res.filesystem.file_exists(filename)
}

pub fn save_response(res: &mut Resources<impl RP>, response: &TemplateResponse) -> Result<()> {
    res.filesystem.write_file(
        &res.filesystem.decode_base64(&response.template_source)?,
        &response.filename,
    )
}
