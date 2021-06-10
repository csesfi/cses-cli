use super::require_login;
use crate::api::CodeSubmit;
use crate::command;
use crate::entities::{SubmissionInfo, SubmitParameters};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{anyhow, Context, Result};

pub fn create_submit_parameters(
    res: &mut Resources<impl RP>,
    parameters: &command::Submit,
) -> Result<SubmitParameters> {
    let storage = res.storage.get_mut();
    if let Some(ref course_id) = parameters.course_id {
        storage.set_course(course_id.clone());
    }
    res.storage.save()?;
    let storage = res.storage.get();
    Ok(SubmitParameters {
        course: storage
            .get_course()
            .ok_or_else(|| anyhow!("Course not provided"))?
            .to_owned(),
        file: parameters.file_name.clone(),
        task: parameters.task_id,
        language: parameters.language.clone(),
    })
}

pub fn submit(
    res: &mut Resources<impl RP>,
    submit_parameters: SubmitParameters,
) -> Result<SubmissionInfo> {
    (|| -> Result<_> {
        require_login(res)?;
        let course_id = submit_parameters.course;
        let task_id = submit_parameters.task;
        let content = res.filesystem.get_file(&submit_parameters.file)?;
        let filename = res.filesystem.get_file_name(&submit_parameters.file)?;
        let content = res.filesystem.encode_base64(&content);
        let submission = CodeSubmit {
            language: submit_parameters.language,
            filename,
            content,
        };
        Ok(res
            .api
            .submit_task(require_login(res)?, &course_id, task_id, &submission)?)
    })()
    .context("Failed submitting file")
}

pub fn submission_info(
    res: &mut Resources<impl RP>,
    submission_id: u64,
    poll: bool,
) -> Result<SubmissionInfo> {
    (|| -> Result<_> {
        let storage = res.storage.get();
        let course_id = storage.get_course().unwrap();
        Ok(res
            .api
            .get_submit(require_login(res)?, course_id, submission_id, poll)?)
    })()
    .context("Failed querying submission status from the server")
}
