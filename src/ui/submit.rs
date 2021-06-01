use crate::{command, service, RP};
use anyhow::Result;

use super::Ui;

pub fn submit(ui: &mut Ui<impl RP>, params: command::Submit) -> Result<(u64)> {
    service::update_submit_parameters(&mut ui.res, &params)?;
    let submission_id = service::submit(&mut ui.res, params.file_name)?;
    Ok(submission_id)
}
