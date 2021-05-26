use crate::service::Login;
use thiserror::Error;
use miniserde::{json, /*Serialize,*/ Deserialize};

pub struct CsesHttpApi {
    URL: String,
}

impl CsesHttpApi {
    pub fn new() -> Self {
        Self {
            URL: "http://127.0.0.1:4010/".to_string(),
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
}

pub type ApiResult<T> = Result<T, ApiError>;

pub trait CsesApi {
    fn login(&self, login: &Login) -> ApiResult<String>;
}

impl CsesApi for CsesHttpApi {
    fn login(&self, login: &Login) -> ApiResult<String> {
        let response = minreq::post(format!("{}/login", self.URL))
            .with_body(json::to_string(login))
            .with_header("Content-Type", "application/json")
            .send()?;
        println!("{:?}", response.as_str());
        let response_body: LoginResponse = json::from_str(response.as_str()?)?;
        let token = response_body.x_auth_token;
        Ok(token)
    }
}

#[derive(Deserialize)]
struct LoginResponse {
    #[serde(rename = "X-Auth-Token")]
    x_auth_token: String,
}