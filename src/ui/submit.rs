use crate::{command, entities::SubmissionResponse, service, RP};
use anyhow::Result;

use super::Ui;

pub fn submit(ui: &mut Ui<impl RP>, params: command::Submit) -> Result<SubmissionResponse> {
    service::update_submit_parameters(&mut ui.res, &params)?;
    let submission_response = service::submit(&mut ui.res, params.file_name)?;
    Ok(submission_response)
}
