use miniserde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub option: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionInfo {
    pub time: String,
    pub language: String,
    pub status: String,
    pub pending: bool,
}
