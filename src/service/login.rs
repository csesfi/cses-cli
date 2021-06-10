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

/// Checks if a session is active, disregarding whether it is still valid
pub fn login_exists(res: &Resources<impl RP>) -> bool {
    res.storage.get().get_token().is_some()
}

pub fn login_status(res: &Resources<impl RP>) -> Result<LoginStatus> {
    if !login_exists(res) {
        return Ok(LoginStatus::Missing);
    }
    match res.api.login_status(res.storage.get().get_token().unwrap()) {
        Err(ApiError::PendingApiKeyError) => return Ok(LoginStatus::Pending),
        Err(ApiError::ApiKeyError) => return Ok(LoginStatus::Invalid),
        val => val?,
    };
    let user = String::from("username"); // TODO
    Ok(LoginStatus::Valid(user))
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
            LoginStatus::Missing => write!(f, "Not logged in, please login."),
            LoginStatus::Pending => write!(f, "Login being finished in browser, please wait."),
            LoginStatus::Invalid => write!(f, "Login is invalid, please login again."),
            LoginStatus::Valid(username) => write!(f, "Logged in as {}", username),
        }
    }
}
