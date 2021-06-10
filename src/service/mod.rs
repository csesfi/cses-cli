mod login;
pub use login::{login, login_exists, login_status, logout, Login};

pub(crate) mod submit;
pub use submit::{submission_info, submit, update_submit_parameters};

use crate::{Resources, Storage, RP};
use anyhow::{anyhow, Result};

pub fn ping(_res: &mut Resources<impl RP>) -> bool {
    true
}

fn require_login(res: &Resources<impl RP>) -> Result<&str> {
    res.storage
        .get()
        .get_token()
        .ok_or_else(|| anyhow!("Not currently logged in"))
}
