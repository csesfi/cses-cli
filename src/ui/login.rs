use crate::service;
use crate::service::Login;
use crate::RP;
use anyhow::Result;

use anyhow::Context;

use super::Ui;

pub fn login(ui: &mut Ui<impl RP>) -> Result<()> {
    try_login(ui).context("Login failed!")
}
fn try_login(ui: &mut Ui<impl RP>) -> Result<()> {
    if service::login_exists(&ui.res) {
        if !prompt_overwrite(ui)? {
            return Ok(());
        }
    }

    let login = Login {
        username: prompt_username(ui)?,
        password: prompt_password(ui)?,
    };
    service::login(&mut ui.res, &login)?;
    ui.term.write_line("Login successful")?;
    Ok(())
}

pub fn logout(ui: &mut Ui<impl RP>) -> Result<()> {
    service::logout(&mut ui.res)?;
    ui.term.write_line("Login invalidated successfully")?;
    Ok(())
}

fn prompt_username(ui: &mut Ui<impl RP>) -> Result<String> {
    ui.term.write_str("Username: ")?;
    ui.prompt_line().context("Failed reading username")
}

fn prompt_password(ui: &mut Ui<impl RP>) -> Result<String> {
    ui.term.write_str("Password: ")?;
    ui.prompt_secure_line().context("Failed reading password")
}

fn prompt_overwrite(ui: &mut Ui<impl RP>) -> Result<bool> {
    ui.term.write_str("Already logged in. Are you sure you want to overwrite the current login session (yes/No)? ")?;
    let answer = ui.prompt_line().context("Failed reading confirmation")?;
    Ok(answer == "yes")
}
