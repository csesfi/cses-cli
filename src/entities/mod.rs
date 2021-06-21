mod course;
pub use course::*;
mod scope;
pub use scope::*;

use miniserde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Language {
    pub name: Option<String>,
    pub option: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UserOutline {
    pub id: u64,
    pub username: String,
    pub displayname: Option<String>,
}

impl UserOutline {
    pub fn name(&self) -> &str {
        self.displayname.as_deref().unwrap_or(&self.username)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct SubmissionInfo {
    pub id: u64,
    pub task: TaskOutline,
    pub sender: UserOutline,
    pub time: String,
    pub language: Language,
    pub status: String,
    pub pending: bool,
    pub test_progress: Option<TestProgress>,
    pub result: Option<String>,
    pub tests: Option<Vec<SubmissionTestInfo>>,
    pub compiler: Option<String>,
    pub test_report: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionTestInfo {
    pub number: u64,
    pub verdict: String,
    pub time: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct TestProgress {
    pub finished_tests: u64,
    pub total_tests: u64,
}

#[derive(Debug, Default, Deserialize)]
pub struct TaskOutline {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitParameters {
    pub course: String,
    pub file: String,
    pub task: Option<u64>,
    pub language: Language,
}

#[derive(Debug)]
pub struct TemplateParameters {
    pub course: String,
    pub task: Option<u64>,
    pub language: Option<String>,
    pub file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CourseInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CourseList {
    pub courses: Vec<CourseInfo>,
}

#[derive(Debug, Deserialize)]
pub struct TemplateResponse {
    pub template_source: String,
    pub filename: String,
}
#[derive(Debug, Deserialize)]
pub struct SubmissionListingInfo {
    pub id: u64,
    pub time: String,
    pub language: Language,
    pub code_time: Option<u64>,
    pub size: Option<u64>,
    pub result: CourseTaskStatus,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionList {
    pub submissions: Vec<SubmissionListingInfo>,
}
