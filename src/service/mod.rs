use crate::{Resources, RP};
use anyhow::Result;

pub struct Login {
    pub username: String,
    pub password: String,
}

pub fn ping(_res: &mut Resources<impl RP>) -> bool {
    true
}

pub fn login(_res: &mut Resources<impl RP>, _login: &Login) -> Result<()> {
    Ok(())
}
