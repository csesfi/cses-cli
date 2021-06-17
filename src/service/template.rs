use crate::command;
use crate::entities::{TemplateParameters, TemplateResponse};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{anyhow, Context, Result};

pub fn create_template_parameters(
    res: &mut Resources<impl RP>,
    parameters: &command::Template,
) -> Result<TemplateParameters> {
    let storage = res.storage.get_mut();
    if let Some(ref course_id) = parameters.course_id {
        storage.set_course(course_id.clone());
    }
    res.storage.save()?;
    let storage = res.storage.get();
    Ok(TemplateParameters {
        course: storage
            .get_course()
            .ok_or_else(|| anyhow!("Course not provided"))?
            .to_owned(),
        file: parameters.file_name.clone(),
        task: parameters.task_id,
        language: parameters.language.clone(),
    })
}

pub fn get_template(
    res: &mut Resources<impl RP>,
    parameters: &TemplateParameters,
) -> Result<TemplateResponse> {
    (|| -> Result<_> {
        let token = res.storage.get().get_token();
        Ok(res.api.get_template(
            token,
            &parameters.course,
            parameters.task,
            parameters.language.as_deref(),
            parameters.file.as_deref(),
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
        &response.file_name,
    )
}
