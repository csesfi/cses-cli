use crate::{command, service, RP};
use anyhow::Result;

use super::Ui;

pub fn submit(ui: &mut Ui<impl RP>, params: command::Submit) -> Result<()> {
    service::update_submit_parameters(&mut ui.res, &params)?;
    let submission_id = service::submit(&mut ui.res, params.file_name)?;
    let long_poll = false;
    let submission_info = service::submission_info(&mut ui.res, submission_id, long_poll)?;
    ui.term.write_line(&submission_info.status)?;
    Ok(())
}
