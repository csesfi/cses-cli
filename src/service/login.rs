use crate::api::ApiError;
use crate::{CsesApi, Resources, Storage, RP};
use anyhow::{Context, Result};
use miniserde::{Deserialize, Serialize};
use std::fmt;

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

pub fn login_status(res: &Resources<impl RP>) -> Result<LoginStatus> {
    if let Some(token) = res.storage.get().get_token() {
        let user = match res.api.login_status(token) {
            Err(ApiError::PendingApiKeyError) => return Ok(LoginStatus::Pending),
            Err(ApiError::ApiKeyError) => return Ok(LoginStatus::Invalid),
            val => val?,
        };
        let name = user.name().to_owned();
        return Ok(LoginStatus::Valid(name));
    }
    Ok(LoginStatus::Missing)
}

pub fn login_is_valid(res: &Resources<impl RP>) -> Result<bool> {
    let status = login_status(res)?;
    Ok(matches!(status, LoginStatus::Valid(_)))
}

pub enum LoginStatus {
    Missing,
    Pending,
    Invalid,
    Valid(String),
}
impl fmt::Display for LoginStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoginStatus::Missing => write!(f, "Not logged in."),
            LoginStatus::Pending => write!(f, "Login waiting to be finished in browser."),
            LoginStatus::Invalid => write!(f, "Login is invalid, please login again."),
            LoginStatus::Valid(username) => write!(f, "Logged in as {}.", username),
        }
    }
}
