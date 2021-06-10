use crate::Storage;
use crate::service;
use crate::RP;
use anyhow::Result;
use std::io::Write;

use anyhow::Context;

use super::Ui;

pub fn login(ui: &mut Ui<impl RP>) -> Result<()> {
    try_login(ui).context("Login failed!")
}
fn try_login(ui: &mut Ui<impl RP>) -> Result<()> {
    if service::login_is_valid(&ui.res)? && !prompt_overwrite(ui)? {
        return Ok(());
    }

    let login_url = service::login(&mut ui.res)?;
    writeln!(ui.term, "Saving token to {}\n\nPlease visit\n{}\nto log in", ui.res.storage.get_path().display(), login_url)?;
    Ok(())
}

pub fn logout(ui: &mut Ui<impl RP>) -> Result<()> {
    service::logout(&mut ui.res)?;
    ui.term.write_line("Login invalidated successfully")?;
    Ok(())
}

fn prompt_overwrite(ui: &mut Ui<impl RP>) -> Result<bool> {
    ui.term.write_str("Already logged in. Are you sure you want to overwrite the current login session (yes/No)? ")?;
    let answer = ui.prompt_line().context("Failed reading confirmation")?;
    Ok(answer == "yes")
}

pub fn status(ui: &mut Ui<impl RP>) -> Result<()> {
    let login_status = service::login_status(&ui.res)?;
    writeln!(ui.term, "Login status: {}", login_status)?;
    Ok(())
}
