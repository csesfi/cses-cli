use crate::service::Login;
use miniserde::{json, /*Serialize,*/ Deserialize};
use minreq::Response;
use thiserror::Error;

pub struct CsesHttpApi {
    url: String,
}

impl CsesHttpApi {
    pub fn new() -> Self {
        Self {
            url: "http://127.0.0.1:4010".to_string(),
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
    #[error("{}", .0.message)]
    ResponseError(ErrorResponse),
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
        check_error(&response)?;
        let response_body: LoginResponse = json::from_str(response.as_str()?)?;
        let token = response_body.x_auth_token;
        Ok(token)
    }

    fn logout(&self, token: &str) -> ApiResult<()> {
        let response = minreq::post(format!("{}/logout", self.url))
            .with_header("X-Auth-Token", token)
            .send()?;
        check_error(&response)?;
        Ok(())
    }
}

fn check_error(response: &Response) -> ApiResult<()> {
    if successful_response(response) {
        Ok(())
    } else {
        let error: ErrorResponse = json::from_str(response.as_str()?)?;
        Err(ApiError::ResponseError(error))
    }
}

fn successful_response(response: &Response) -> bool {
    (200..300).contains(&response.status_code)
}

#[derive(Deserialize)]
struct LoginResponse {
    #[serde(rename = "X-Auth-Token")]
    x_auth_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: ErrorCode,
}

#[derive(Debug, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "invalid_api_key")]
    InvalidApiKey,
    #[serde(rename = "invalid_credentials")]
    InvalidCredentials,
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "client_error")]
    ClientError,
}
