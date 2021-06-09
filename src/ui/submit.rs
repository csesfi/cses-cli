use crate::{command, entities::SubmissionResponse, service, RP};
use anyhow::Result;

use super::Ui;

pub fn submit(ui: &mut Ui<impl RP>, params: command::Submit) -> Result<SubmissionResponse> {
    let submit_params = service::create_submit_parameters(&mut ui.res, &params)?;
    let submission_response = service::submit(&mut ui.res, submit_params)?;
    Ok(submission_response)
}
