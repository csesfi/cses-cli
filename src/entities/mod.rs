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
    pub result: Option<String>,
    pub tests: Option<Vec<SubmissionTestInfo>>,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionTestInfo {
    number: u64,
    verdict: String,
    time: u64,
}
