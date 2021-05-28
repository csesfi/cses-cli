use crate::{CsesApi, Resources, Storage, RP};
use anyhow::{anyhow, Result};
use miniserde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub fn ping(_res: &mut Resources<impl RP>) -> bool {
    true
}

pub fn login(res: &mut Resources<impl RP>, login: &Login) -> Result<()> {
    let token = res.api.login(login)?;
    res.storage.set_token(token);
    res.storage.save()
}

pub fn logout(res: &mut Resources<impl RP>) -> Result<()> {
    if let Some(token) = res.storage.get_token() {
        res.api.logout(token)?;
        res.storage.delete()?;
        Ok(())
    } else {
        Err(anyhow!("not currently logged in"))
    }
}
