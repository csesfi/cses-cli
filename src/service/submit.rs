use super::require_login;
use crate::api::CodeSubmit;
use crate::command;
use crate::entities::{Scope, SubmissionInfo, SubmissionListingInfo};
use crate::{CsesApi, Filesystem, Resources, RP};
use anyhow::{anyhow, Context, Result};

pub fn submit(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    parameters: command::Submit,
) -> Result<SubmissionInfo> {
    (|| -> Result<_> {
        require_login(res)?;
        let task = parameters.task.as_deref();
        let content = res.filesystem.get_file(&parameters.file_name)?;
        let filename = res.filesystem.get_file_name(&parameters.file_name)?;
        let content = res.filesystem.encode_base64(&content);
        let submission = CodeSubmit {
            language: parameters.language,
            filename,
            content,
        };
        Ok(res
            .api
            .submit_task(require_login(res)?, &scope, task, &submission)?)
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
        Ok(res
            .api
            .get_submit(require_login(res)?, &scope, submission_id, poll)?)
    })()
    .context("Failed querying submission status from the server")
}

pub fn submission_list(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    task_id: &str,
) -> Result<Vec<SubmissionListingInfo>> {
    (|| -> Result<_> {
        let response = res
            .api
            .get_submit_list(require_login(res)?, &scope, task_id)?;
        Ok(response.submissions)
    })()
    .context("Failed querying submissions from the server")
}

pub fn nth_last_submission_info(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    task_id: &str,
    nth_last: u64,
) -> Result<SubmissionInfo> {
    (|| {
        let submissions = submission_list(res, scope, task_id)?;
        let n_submissions = submissions.len();
        let submission_id = (|| submissions.get(n_submissions.checked_sub(nth_last as usize)?))()
            .ok_or_else(|| {
                anyhow!(format!(
                    "The nth last submission doesn't exist for n = {}",
                    nth_last
                ))
            })?
            .id;
        submission_info(res, scope, submission_id, false)
    })()
    .context("Failed fetching the nth submission")
}
