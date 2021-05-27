use crate::service;
use crate::service::Login;
use crate::RP;
use anyhow::Result;
use console::Term;

use anyhow::Context;

use super::Ui;

pub fn login(ui: &mut Ui<impl RP>) -> Result<()> {
    try_login(ui).context("Login failed!")
}
fn try_login(ui: &mut Ui<impl RP>) -> Result<()> {
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
