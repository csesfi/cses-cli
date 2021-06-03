use crate::api::ApiError;
use crate::{CsesApi, Resources, Storage, RP};
use anyhow::{Context, Result};
use miniserde::{Deserialize, Serialize};

use super::require_login;

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub fn login(res: &mut Resources<impl RP>, login: &Login) -> Result<()> {
    let token = res.api.login(login)?;
    res.storage.get_mut().set_token(token);
    res.storage.save()?;
    Ok(())
}

pub fn logout(res: &mut Resources<impl RP>) -> Result<()> {
    (|| -> Result<_> {
        let token = require_login(res)?;
        // Invalid API key error can be ignored because the goal of the server
        // communication in logout is to invalidate the API key
        match res.api.logout(token) {
            Err(ApiError::ApiKeyError) => (),
            val => val?,
        };
        res.storage.delete()?;
        Ok(())
    })()
    .context("Failed to log out")
}

/// Checks if a session is active, disregarding whether it is still valid
pub fn login_exists(res: &Resources<impl RP>) -> bool {
    res.storage.get().get_token().is_some()
}
