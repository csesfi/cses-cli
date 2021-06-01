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
        let confirmation = prompt_overwrite(ui).context("Failed reading confirmation")?;
        if confirmation.to_lowercase() != "yes" {
            return Ok(());
        }
    }

    let login = Login {
        username: prompt_username(ui).context("Failed reading username")?,
        password: prompt_password(ui).context("Failed reading password")?,
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
    ui.prompt_line()
}

fn prompt_password(ui: &mut Ui<impl RP>) -> Result<String> {
    ui.term.write_str("Password: ")?;
    ui.prompt_secure_line()
}

fn prompt_overwrite(ui: &mut Ui<impl RP>) -> Result<String> {
    ui.term.write_str("Already logged in. Are you sure you want to overwrite the current login session (yes/No)? ")?;
    ui.prompt_line()
}
