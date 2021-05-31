mod login;
pub use login::{login, logout, Login};

mod submit;
pub use submit::{submission_info, submit};

use crate::{Resources, Storage, RP};
use anyhow::{anyhow, Result};

pub fn ping(_res: &mut Resources<impl RP>) -> bool {
    true
}

fn require_login(res: &mut Resources<impl RP>) -> Result<&str> {
    res.storage
        .get_token()
        .ok_or(anyhow!("Not currently logged in"))
}
