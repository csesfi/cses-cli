mod courses;
mod login;
mod statement;
mod template;
mod test_case;
pub use courses::{courses, scope_content};
pub use login::{login, login_is_valid, login_status, logout, Login};
pub use statement::get_task_statement;
pub use template::{file_exists, get_template, save_response};
pub use test_case::{create_dir_all, fetch_samples, save_test_cases, test_cases_exist};

pub mod submit;
pub use submit::{nth_latest_submission_info, submission_info, submission_list, submit};

mod scope;
use anyhow::{anyhow, Result};
pub use scope::select_scope;

use crate::{Resources, Storage, RP};

fn require_login(res: &Resources<impl RP>) -> Result<&str> {
    res.storage
        .get()
        .get_token()
        .ok_or_else(|| anyhow!("Not currently logged in"))
}
