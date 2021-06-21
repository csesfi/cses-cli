use crate::{
    command,
    entities::{Scope, SubmissionInfo},
    service, RP,
};
use anyhow::Result;

use super::Ui;

pub fn submit(
    ui: &mut Ui<impl RP>,
    scope: Scope,
    params: command::Submit,
) -> Result<SubmissionInfo> {
    let submit_params = service::create_submit_parameters(&mut ui.res, scope, params)?;
    let submission_response = service::submit(&mut ui.res, submit_params)?;
    Ok(submission_response)
}
