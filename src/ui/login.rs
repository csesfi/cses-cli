use crate::service;
use crate::Storage;
use crate::RP;
use anyhow::Result;
use std::io::Write;

use anyhow::Context;

use super::{prompt_yes_no, Ui};

pub fn login(ui: &mut Ui<impl RP>) -> Result<()> {
    (|| -> Result<_> {
        let overwrite_message = "Already logged in. Are you sure you want to overwrite the current login session? (yes/No) ";
        if service::login_is_valid(&ui.res)? && !prompt_yes_no(ui, overwrite_message)? {
            return Ok(());
        }

        let login_url = service::login(&mut ui.res)?;
        writeln!(
            ui.term,
            "Saving token to {}\n\nPlease visit\n{}\nto log in",
            ui.res.storage.get_path().display(),
            login_url
        )?;
        Ok(())
    })().context("Login failed!")
}

pub fn logout(ui: &mut Ui<impl RP>) -> Result<()> {
    service::logout(&mut ui.res)?;
    ui.term.write_line("Login invalidated successfully")?;
    Ok(())
}

pub fn status(ui: &mut Ui<impl RP>) -> Result<()> {
    let login_status = service::login_status(&ui.res)?;
    writeln!(ui.term, "Login status: {}", login_status)?;
    Ok(())
}
