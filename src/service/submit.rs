use super::require_login;
use crate::api::CodeSubmit;
use crate::command;
use crate::entities::{Scope, SubmissionInfo, SubmissionListingInfo, SubmitParameters, TaskId};
use crate::{CsesApi, Filesystem, Resources, RP};
use anyhow::{Context, Result};

pub fn create_submit_parameters(
    _res: &mut Resources<impl RP>,
    scope: &Scope,
    parameters: command::Submit,
) -> Result<SubmitParameters> {
    // FIXME
    let course = match scope {
        Scope::Course(course) => course,
        _ => panic!(),
    };
    Ok(SubmitParameters {
        course: course.to_string(),
        file: parameters.file_name,
        task: parameters.task,
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
        // FIXME
        let task = submit_parameters.task.map(|t| match t {
            TaskId::Number(id) => id,
            _ => panic!(),
        });
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
            .submit_task(require_login(res)?, &course_id, task, &submission)?)
    })()
    .context("Failed submitting file")
}

pub fn submission_info(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    submission_id: u64,
    poll: bool,
) -> Result<SubmissionInfo> {
    (|| -> Result<_> {
        // FIXME
        let course_id = match scope {
            Scope::Course(course) => course,
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
    scope: &Scope,
    task_id: u64,
) -> Result<Vec<SubmissionListingInfo>> {
    (|| -> Result<_> {
        // FIXME
        let course_id = match scope {
            Scope::Course(course) => course,
            _ => panic!(),
        };
        let response = res
            .api
            .get_submit_list(require_login(res)?, &course_id, task_id)?;
        Ok(response.submissions)
    })()
    .context("Failed querying submissions from the server")
}
