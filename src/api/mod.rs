use crate::service::Login;
use miniserde::{json, /*Serialize,*/ Deserialize};
use thiserror::Error;

pub struct CsesHttpApi {
    url: String,
}

impl CsesHttpApi {
    pub fn new() -> Self {
        Self {
            url: "http://127.0.0.1:4010/".to_string(),
        }
    }
}

impl Default for CsesHttpApi {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error")]
    HttpError(#[from] minreq::Error),
    #[error("JSON error")]
    JsonError(#[from] miniserde::Error),
    #[error("{0}")]
    CustomError(String),
}

pub type ApiResult<T> = Result<T, ApiError>;

pub trait CsesApi {
    fn login(&self, login: &Login) -> ApiResult<String>;
    fn logout(&self, token: &str) -> ApiResult<()>;
}

impl CsesApi for CsesHttpApi {
    fn login(&self, login: &Login) -> ApiResult<String> {
        let response = minreq::post(format!("{}/login", self.url))
            .with_body(json::to_string(login))
            .with_header("Content-Type", "application/json")
            .send()?;
        let success = (200..300).contains(&response.status_code);
        if success {
            let response_body: LoginResponse = json::from_str(response.as_str()?)?;
            let token = response_body.x_auth_token;
            Ok(token)
        } else {
            let message: String = json::from_str(response.as_str()?)?;
            Err(ApiError::CustomError(message))
        }
    }

    fn logout(&self, _token: &str) -> ApiResult<()> {
        todo!()
    }
}

#[derive(Deserialize)]
struct LoginResponse {
    #[serde(rename = "X-Auth-Token")]
    x_auth_token: String,
}
