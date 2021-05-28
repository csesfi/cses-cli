use miniserde::{json, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub option: Option<String>,
}
