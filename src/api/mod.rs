mod escape;
mod trace_send;
use crate::entities::{
    CourseList, Language, Scope, ScopeContent, SubmissionInfo, SubmissionList, TemplateResponse,
    TestCaseList, UserOutline,
};
use escape::Escape;
use miniserde::{json, Deserialize, Serialize};
use minreq::Response;
#[cfg(test)]
use mockall::automock;
use thiserror::Error;
use trace_send::TraceSend;

pub struct CsesHttpApi {
    url: String,
    trace: bool,
}

impl CsesHttpApi {
    pub fn new(url: String, trace: bool) -> Self {
        Self { url, trace }
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internet connection error")]
    HttpError(#[from] minreq::Error),
    #[error("Could not parse server response")]
    JsonError(#[from] miniserde::Error),
    #[error("API key pending authentication.")]
    PendingApiKeyError,
    #[error("Invalid API key. Log in again.")]
    ApiKeyError,
    #[error("Server error: \"{}\"", .0)]
    ServerError(String),
    #[error("API request failed: \"{}\"", .0)]
    ClientError(String),
    #[error("Task deduction error: \"{}\"", .0)]
    TaskDeductionError(String),
    #[error("Language deduction error: \"{}\"", .0)]
    LanguageDeductionError(String),
}

pub type ApiResult<T> = Result<T, ApiError>;

#[allow(clippy::needless_lifetimes)]
#[cfg_attr(test, automock)]
pub trait CsesApi {
    fn login(&self) -> ApiResult<LoginResponse>;
    fn login_status(&self, token: &str) -> ApiResult<UserOutline>;
    fn logout(&self, token: &str) -> ApiResult<()>;
    fn submit_task<'a>(
        &self,
        token: &str,
        scope: &Scope,
        task_id: Option<&'a str>,
        submission: &CodeSubmit,
    ) -> ApiResult<SubmissionInfo>;
    fn get_submit(
        &self,
        token: &str,
        scope: &Scope,
        submission_id: u64,
        poll: bool,
    ) -> ApiResult<SubmissionInfo>;
    fn get_submit_list(
        &self,
        token: &str,
        scope: &Scope,
        task_id: &str,
    ) -> ApiResult<SubmissionList>;
    fn get_courses<'a>(&self, token: Option<&'a str>) -> ApiResult<CourseList>;
    fn get_content<'a>(&self, token: Option<&'a str>, scope: &Scope) -> ApiResult<ScopeContent>;
    fn get_template<'a>(
        &self,
        token: Option<&'a str>,
        scope: &Scope,
        task_id: Option<&'a str>,
        language: Option<&'a str>,
        file: Option<&'a str>,
    ) -> ApiResult<TemplateResponse>;
    fn get_test_case_list<'a>(
        &self,
        token: Option<&'a str>,
        scope: &Scope,
        task_id: &str,
    ) -> ApiResult<TestCaseList>;
}

impl CsesApi for CsesHttpApi {
    fn login(&self) -> ApiResult<LoginResponse> {
        let response = minreq::post(format!("{}/login", self.url)).trace_send(self.trace)?;
        check_error(&response)?;
        Ok(json::from_str(response.as_str()?)?)
    }

    fn login_status(&self, token: &str) -> ApiResult<UserOutline> {
        let response = minreq::get(format!("{}/login", self.url))
            .with_header("X-Auth-Token", token)
            .trace_send(self.trace)?;
        check_error(&response)?;
        let response: UserOutline = json::from_str(response.as_str()?)?;
        Ok(response)
    }

    fn logout(&self, token: &str) -> ApiResult<()> {
        let response = minreq::post(format!("{}/logout", self.url))
            .with_header("X-Auth-Token", token)
            .trace_send(self.trace)?;
        check_error(&response)?;
        Ok(())
    }

    fn submit_task(
        &self,
        token: &str,
        scope: &Scope,
        task_id: Option<&str>,
        submission: &CodeSubmit,
    ) -> ApiResult<SubmissionInfo> {
        let mut request = minreq::post(format_url(&self.url, scope, "submissions"))
            .with_body(json::to_string(submission))
            .with_header("X-Auth-Token", token)
            .with_header("Content-Type", "application/json");

        if let Some(task_id) = task_id {
            request = request.with_param("task", task_id);
        }

        let response = request.trace_send(self.trace)?;
        check_error(&response)?;
        let response_body: SubmissionInfo = json::from_str(response.as_str()?)?;
        Ok(response_body)
    }

    fn get_submit(
        &self,
        token: &str,
        scope: &Scope,
        submission_id: u64,
        poll: bool,
    ) -> ApiResult<SubmissionInfo> {
        let poll = if poll { "true" } else { "false" };
        let response = minreq::get(format_url(
            &self.url,
            scope,
            &format!("submissions/{}", submission_id),
        ))
        .with_header("X-Auth-Token", token)
        .with_param("poll", poll)
        .trace_send(self.trace)?;
        check_error(&response)?;
        let response_body: SubmissionInfo = json::from_str(response.as_str()?)?;
        Ok(response_body)
    }

    fn get_submit_list(
        &self,
        token: &str,
        scope: &Scope,
        task_id: &str,
    ) -> ApiResult<SubmissionList> {
        let response = minreq::get(format_url(&self.url, scope, "submissions"))
            .with_header("X-Auth-Token", token)
            .with_param("task", task_id)
            .trace_send(self.trace)?;
        check_error(&response)?;
        let response_body: SubmissionList = json::from_str(response.as_str()?)?;
        Ok(response_body)
    }

    fn get_courses(&self, token: Option<&str>) -> ApiResult<CourseList> {
        match token {
            Some(token) => {
                let response = minreq::get(format!("{}/courses", self.url))
                    .with_header("X-Auth-Token", token)
                    .trace_send(self.trace)?;
                check_error(&response)?;
                let course_list: CourseList = json::from_str(response.as_str()?)?;
                Ok(course_list)
            }
            None => {
                let response =
                    minreq::get(format!("{}/courses", self.url)).trace_send(self.trace)?;
                check_error(&response)?;
                let course_list: CourseList = json::from_str(response.as_str()?)?;
                Ok(course_list)
            }
        }
    }

    fn get_content<'a>(&self, token: Option<&'a str>, scope: &Scope) -> ApiResult<ScopeContent> {
        let mut request = minreq::get(format_url(&self.url, scope, "list"));
        if let Some(token) = token {
            request = request.with_header("X-Auth-Token", token);
        }
        let response = request.trace_send(self.trace)?;
        check_error(&response)?;
        let scope_content: ScopeContent = json::from_str(response.as_str()?)?;
        Ok(scope_content)
    }

    fn get_template<'a>(
        &self,
        token: Option<&'a str>,
        scope: &Scope,
        task_id: Option<&'a str>,
        language: Option<&'a str>,
        file_name: Option<&'a str>,
    ) -> ApiResult<TemplateResponse> {
        let mut request = minreq::get(format_url(&self.url, scope, "templates"));
        if let Some(token) = token {
            request = request.with_header("X-Auth-Token", token);
        }
        if let Some(task_id) = task_id {
            request = request.with_param("task", task_id);
        }
        if let Some(language) = language {
            request = request.with_param("language", language);
        }
        if let Some(file_name) = file_name {
            request = request.with_param("filename", file_name);
        }
        let response = request.trace_send(self.trace)?;
        check_error(&response)?;
        Ok(json::from_str(response.as_str()?)?)
    }
    fn get_test_case_list<'a>(
        &self,
        token: Option<&'a str>,
        scope: &Scope,
        task_id: &str,
    ) -> ApiResult<TestCaseList> {
        let mut request =
            minreq::get(format_url(&self.url, scope, "test-cases")).with_param("task", task_id);
        if let Some(token) = token {
            request = request.with_header("X-Auth-Token", token);
        }
        let response = request.trace_send(self.trace)?;
        check_error(&response)?;
        let response_body: TestCaseList = json::from_str(response.as_str()?)?;
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
            ErrorCode::PendingApiKey => ApiError::PendingApiKeyError,
            ErrorCode::ServerError => ApiError::ServerError(error.message),
            ErrorCode::ClientError => ApiError::ClientError(error.message),
            ErrorCode::TaskDeductionError => ApiError::TaskDeductionError(error.message),
            ErrorCode::LanguageDeductionError => ApiError::LanguageDeductionError(error.message),
        })
    }
}

fn successful_response(response: &Response) -> bool {
    (200..300).contains(&response.status_code)
}

fn format_url(base_url: &str, scope: &Scope, path: &str) -> String {
    match scope {
        Scope::Course(course_id) => {
            format!("{}/courses/{}/{}", base_url, Escape(course_id), path)
        }
        Scope::Contest(contest_id) => {
            format!("{}/contests/{}/{}", base_url, contest_id, path)
        }
    }
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
    #[serde(rename = "pending_api_key")]
    PendingApiKey,
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "client_error")]
    ClientError,
    #[serde(rename = "task_deduction_error")]
    TaskDeductionError,
    #[serde(rename = "language_deduction_error")]
    LanguageDeductionError,
}

#[derive(Debug, Serialize)]
pub struct CodeSubmit {
    pub language: Language,
    pub filename: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "X-Auth-Token")]
    pub token: String,
    pub authentication_url: String,
}
