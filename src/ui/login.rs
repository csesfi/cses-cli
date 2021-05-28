use crate::service;
use crate::service::Login;
use crate::storage::Storage;
use crate::RP;
use anyhow::Result;
use console::Term;

use anyhow::Context;

use super::Ui;

pub fn login(ui: &mut Ui<impl RP>) -> Result<()> {
    try_login(ui).context("Login failed!")
}
fn try_login(ui: &mut Ui<impl RP>) -> Result<()> {
    if let Some(_token) = ui.res.storage.get_token() {
        let confirmation = prompt_overwrite(&mut ui.term).context("Failed reading confirmation")?;
        if confirmation.to_lowercase() != "yes" {
            return Ok(());
        }
    }

    let login = Login {
        username: prompt_username(&mut ui.term).context("Failed reading username")?,
        password: prompt_password(&mut ui.term).context("Failed reading password")?,
    };
    service::login(&mut ui.res, &login)?;
    ui.term.write_line("Login successful")?;
    Ok(())
}

fn prompt_username(term: &mut Term) -> Result<String> {
    term.write_str("Username: ")?;
    Ok(term.read_line()?)
}

fn prompt_password(term: &mut Term) -> Result<String> {
    term.write_str("Password: ")?;
    Ok(term.read_secure_line()?)
}

fn prompt_overwrite(term: &mut Term) -> Result<String> {
    term.write_str("Already logged in. Are you sure you want to overwrite the current login session (yes/No)? ")?;
    Ok(term.read_line()?)
}
