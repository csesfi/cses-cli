mod courses;
mod login;
mod template;
pub use courses::courses;
pub use login::{login, login_is_valid, login_status, logout, Login};
pub use template::{create_template_parameters, file_exists, get_template, save_response};

pub(crate) mod submit;
pub use submit::{create_submit_parameters, submission_info, submit};

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
