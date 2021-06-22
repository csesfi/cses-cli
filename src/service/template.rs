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
            // FIXME
            parameters.task.as_deref().map(|t| t.parse().unwrap()),
            parameters.language.as_deref(),
            parameters.file_name.as_deref(),
        )?)
    })()
    .context("Failed querying code template from the server")
}

pub fn file_exists(res: &Resources<impl RP>, file_name: &str) -> bool {
    res.filesystem.file_exists(file_name)
}

pub fn save_response(res: &mut Resources<impl RP>, response: &TemplateResponse) -> Result<()> {
    res.filesystem.write_file(
        &res.filesystem.decode_base64(&response.template_source)?,
        &response.filename,
    )
}
