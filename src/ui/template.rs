use super::{prompt_yes_no, Ui};
use crate::{command, service, RP};
use anyhow::Result;
use std::io::Write;

pub fn get_template(ui: &mut Ui<impl RP>, params: command::Template) -> Result<()> {
    let template_parameters = service::create_template_parameters(&mut ui.res, params)?;
    let template_response = service::get_template(&mut ui.res, &template_parameters)?;
    if service::file_exists(&ui.res, &template_response.filename) {
        let overwrite_message = format!(
            "A file ./{} already exists.\nDo you want to overwrite it \
            with the new template? (yes/No)? ",
            &template_response.filename
        );
        if !prompt_yes_no(ui, &overwrite_message)? {
            return Ok(());
        }
    }
    service::save_response(&mut ui.res, &template_response)?;
    Ok(writeln!(
        ui.term,
        "Template file was successfully saved to ./{}",
        &template_response.filename
    )?)
}
