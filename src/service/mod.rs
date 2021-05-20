use crate::login::Login;
use crate::{Resources, RP};
use anyhow::Result;

pub fn ping(_res: &mut Resources<impl RP>) -> bool {
    true
}

pub fn login(_res: &mut Resources<impl RP>, _login: &Login) -> Result<()> {
    Ok(())
}
