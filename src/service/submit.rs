use anyhow::{anyhow, Context, Result};

use super::require_login;
use crate::api::CodeSubmit;
use crate::entities::{Scope, SubmissionInfo, SubmissionListingInfo};
use crate::{command, CsesApi, Filesystem, Resources, RP};

pub fn submit(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    parameters: command::Submit,
) -> Result<SubmissionInfo> {
    (|| -> Result<_> {
        require_login(res)?;
        let task_id = parameters.task_id.as_deref();
        let content = res.filesystem.get_file(&parameters.filename)?;
        let filename = res.filesystem.get_filename(&parameters.filename)?;
        let content = res.filesystem.encode_base64(&content);
        let submission = CodeSubmit {
            language: parameters.language,
            filename,
            content,
        };
        Ok(res
            .api
            .submit_task(require_login(res)?, scope, task_id, &submission)?)
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
            .get_submit(require_login(res)?, scope, submission_id, poll)?)
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
            .get_submit_list(require_login(res)?, scope, task_id)?;
        Ok(response.submissions)
    })()
    .context("Failed querying submissions from the server")
}

pub fn nth_latest_submission_info(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    task_id: &str,
    n: u64,
) -> Result<SubmissionInfo> {
    (|| {
        let submissions = submission_list(res, scope, task_id)?;
        let idx = submissions
            .len()
            .checked_sub(1 + n as usize)
            .ok_or_else(|| anyhow!("The nth latest submission doesn't exist for n = {}", n))?;
        let submission_id = submissions[idx].id;
        submission_info(res, scope, submission_id, false)
    })()
    .context("Failed fetching the nth latest submission")
}
