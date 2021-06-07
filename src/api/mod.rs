mod escape;
use escape::Escape;

use crate::entities::{Language, SubmissionInfo};
use crate::service::Login;
use miniserde::{json, Deserialize, Serialize};
use minreq::Response;
#[cfg(test)]
use mockall::automock;
use thiserror::Error;

pub struct CsesHttpApi {
    url: String,
}

impl CsesHttpApi {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl Default for CsesHttpApi {
    fn default() -> Self {
        Self::new(String::from("http://127.0.0.1:4010"))
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internet connection error")]
    HttpError(#[from] minreq::Error),
    #[error("Could not parse server response")]
    JsonError(#[from] miniserde::Error),
    #[error("Invalid credentials")]
    InvalidCredentialsError,
    #[error("Invalid API key. Log in again.")]
    ApiKeyError,
    #[error("Server error: \"{}\"", .0)]
    ServerError(String),
    #[error("API request failed: \"{}\"", .0)]
    ClientError(String),
}

pub type ApiResult<T> = Result<T, ApiError>;

#[cfg_attr(test, automock)]
pub trait CsesApi {
    fn login(&self, login: &Login) -> ApiResult<String>;
    fn logout(&self, token: &str) -> ApiResult<()>;
    fn submit_task(
        &self,
        token: &str,
        course_id: &str,
        task_id: u64,
        submission: &CodeSubmit,
    ) -> ApiResult<u64>;
    fn get_submit(
        &self,
        token: &str,
        course_id: &str,
        task_id: u64,
        submission_id: u64,
        poll: bool,
    ) -> ApiResult<SubmissionInfo>;
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

    fn submit_task(
        &self,
        token: &str,
        course_id: &str,
        task_id: u64,
        submission: &CodeSubmit,
    ) -> ApiResult<u64> {
        let response = minreq::post(format!(
            "{}/courses/{}/tasks/{}/submissions",
            self.url,
            Escape(course_id),
            task_id
        ))
        .with_body(json::to_string(submission))
        .with_header("X-Auth-Token", token)
        .with_header("Content-Type", "application/json")
        .send()?;
        check_error(&response)?;
        let response_body: SubmissionResponse = json::from_str(response.as_str()?)?;
        let submission_id = response_body.id;
        Ok(submission_id)
    }

    fn get_submit(
        &self,
        token: &str,
        course_id: &str,
        task_id: u64,
        submission_id: u64,
        poll: bool,
    ) -> ApiResult<SubmissionInfo> {
        let poll = if poll { "true" } else { "false" };
        let response = minreq::get(format!(
            "{}/courses/{}/tasks/{}/submissions/{}?poll={}",
            self.url,
            Escape(course_id),
            task_id,
            submission_id,
            poll
        ))
        .with_header("X-Auth-Token", token)
        .send()?;
        check_error(&response)?;
        let response_body: SubmissionInfo = json::from_str(response.as_str()?)?;
        Ok(response_body)
    }
}

fn check_error(response: &Response) -> ApiResult<()> {
    if successful_response(response) {
        Ok(())
    } else {
        let error: ErrorResponse = json::from_str(response.as_str()?)?;
        Err(match error.code {
            ErrorCode::InvalidApiKey => ApiError::ApiKeyError,
            ErrorCode::InvalidCredentials => ApiError::InvalidCredentialsError,
            ErrorCode::ServerError => ApiError::ServerError(error.message),
            ErrorCode::ClientError => ApiError::ClientError(error.message),
        })
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

#[derive(Deserialize)]
struct SubmissionResponse {
    id: u64,
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

#[derive(Debug, Serialize)]
pub struct CodeSubmit {
    pub language: Language,
    pub filename: String,
    pub content: String,
}
