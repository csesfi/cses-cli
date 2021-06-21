use super::require_login;
use crate::api::CodeSubmit;
use crate::command;
use crate::entities::{Scope, SubmissionInfo, SubmissionListingInfo, SubmitParameters};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{Context, Result};

pub fn create_submit_parameters(
    _res: &mut Resources<impl RP>,
    course_id: String,
    parameters: command::Submit,
) -> Result<SubmitParameters> {
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
        // FIXME
        let course_id = match storage.get_scope() {
            Some(Scope::Course(course)) => course,
            _ => panic!(),
        };
        Ok(res
            .api
            .get_submit(require_login(res)?, &course_id, submission_id, poll)?)
    })()
    .context("Failed querying submission status from the server")
}

pub fn submission_list(
    res: &mut Resources<impl RP>,
    task_id: u64,
) -> Result<Vec<SubmissionListingInfo>> {
    (|| -> Result<_> {
        let storage = res.storage.get();
        // FIXME
        let course_id = match storage.get_scope() {
            Some(Scope::Course(course)) => course,
            _ => panic!(),
        };
        let response = res
            .api
            .get_submit_list(require_login(res)?, &course_id, task_id)?;
        Ok(response.submissions)
    })()
    .context("Failed querying submissions from the server")
}
