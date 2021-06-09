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

pub fn login(res: &mut Resources<impl RP>) -> Result<String> {
    let login_response = res.api.login()?;
    res.storage.get_mut().set_token(login_response.token);
    res.storage.save()?;
    Ok(login_response.authentication_url)
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

// Returns true if token is valid and false if not or is missing.
pub fn login_status(res: &Resources<impl RP>) -> Result<bool> {
    if !login_exists(res) {
        return Ok(false);
    }
    match res.api.login_status(res.storage.get().get_token().unwrap()) {
        Err(ApiError::PendingApiKeyError) | Err(ApiError::ApiKeyError) => return Ok(false),
        val => val?,
    };
    Ok(true)
}
