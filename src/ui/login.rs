use crate::service;
use crate::service::Login;
use crate::RP;
use anyhow::Result;
use console::Term;

use super::Ui;

pub fn login(ui: &mut Ui<impl RP>) -> Result<()> {
    let login = Login {
        username: prompt_username(&mut ui.term)?,
        password: prompt_password(&mut ui.term)?,
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
