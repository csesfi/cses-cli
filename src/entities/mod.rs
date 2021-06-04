use miniserde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub option: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionInfo {
    pub time: String,
    pub language: Language,
    pub status: String,
    pub pending: bool,
    pub test_progress: Option<TestProgress>,
    pub result: Option<String>,
    pub tests: Option<Vec<SubmissionTestInfo>>,
    pub compiler: Option<String>,
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
