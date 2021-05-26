use crate::{CsesApi, Resources, Storage, RP};
use anyhow::Result;

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
