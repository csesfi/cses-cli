use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR};

use anyhow::Result;

use super::util::prompt_yes_no;
use super::Ui;
use crate::entities::Scope;
use crate::{command, service, RP};

pub fn get_template(ui: &mut Ui<impl RP>, scope: &Scope, params: command::Template) -> Result<()> {
    let template_response = service::get_template(&mut ui.res, scope, params)?;
    if service::file_exists(&ui.res, Path::new(&template_response.filename)) {
        let overwrite_message = format!(
            "A file .{}{} already exists.\n\
            Do you want to overwrite it with the new template? (yes/No) ",
            MAIN_SEPARATOR, &template_response.filename
        );
        if !prompt_yes_no(ui, &overwrite_message)? {
            return Ok(());
        }
    }
    service::save_response(&mut ui.res, &template_response)?;
    Ok(writeln!(
        ui.term,
        "Template file was successfully saved to .{}{}",
        MAIN_SEPARATOR, &template_response.filename
    )?)
}
