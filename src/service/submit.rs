use super::require_login;
use crate::api::CodeSubmit;
use crate::command;
use crate::entities::{SubmissionInfo, SubmissionList, SubmitParameters};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{Context, Result};

pub fn create_submit_parameters(
    res: &mut Resources<impl RP>,
    parameters: command::Submit,
) -> Result<SubmitParameters> {
    let course_id = super::select_course(res, parameters.course_id)?;
    Ok(SubmitParameters {
        course: course_id,
        file: parameters.file_name,
        task: parameters.task_id,
        language: parameters.language,
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

pub fn submission_list(res: &mut Resources<impl RP>, task_id: u64) -> Result<SubmissionList> {
    (|| -> Result<_> {
        let storage = res.storage.get();
        let course_id = storage.get_course().unwrap();
        Ok(res
            .api
            .get_submit_list(require_login(res)?, course_id, task_id)?)
    })()
    .context("Failed querying submissions from the server")
}
