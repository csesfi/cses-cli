use std::{fmt, num::ParseIntError, str::FromStr};
use anyhow::anyhow;
use miniserde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Course(String),
    Contest(u64),
}

impl FromStr for Scope {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let try_number = s.parse();
        Ok(match try_number {
            Ok(number) => Scope::Contest(number),
            Err(_) => Scope::Course(s.to_owned()),
        })
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Course(id) => write!(f, "{}", id)?,
            Scope::Contest(id) => write!(f, "{}", id)?,
        }
        Ok(())
    }
}


#[derive(Debug, Deserialize)]
pub struct ScopeContent {
    pub sections: Vec<ScopeSection>,
}

#[derive(Debug, Deserialize)]
pub struct ScopeSection {
    pub header: String,
    pub text: Option<String>,
    pub list: Vec<ScopeItemRaw>,
}

#[derive(Debug)]
pub enum ScopeItem<'a> {
    Text {
        name: &'a str,
        id: String,
        link: &'a str,
    },
    Link {
        name: &'a str,
        link: &'a str,
    },
    Task {
        name: &'a str,
        id: String,
        link: &'a str,
        status: Option<TaskStatus>,
        score: Option<u64>,
    },
}

#[derive(Debug, Deserialize)]
pub struct ScopeItemRaw {
    #[serde(rename = "objectType")]
    object_type: ScopeItemType,
    name: String,
    id: Option<String>,
    link: String,
    status: Option<TaskStatus>,
    score: Option<u64>,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum TaskStatus {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Deserialize)]
pub enum ScopeItemType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "task")]
    Task,
}

impl ScopeItemRaw {
    pub fn as_enum(&self) -> anyhow::Result<ScopeItem<'_>> {
        Ok(match &self.object_type {
            ScopeItemType::Text => ScopeItem::Text {
                name: &self.name,
                id: self.id.clone().ok_or_else(|| anyhow!("Could not get ID"))?,
                link: &self.link,
            },
            ScopeItemType::Link => ScopeItem::Link {
                name: &self.name,
                link: &self.link,
            },
            ScopeItemType::Task => ScopeItem::Task {
                name: &self.name,
                id: self.id.clone().ok_or_else(|| anyhow!("Could not get ID"))?,
                link: &self.link,
                status: self.status,
                score: self.score,
            },
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_course() {
        let scope = "text123".parse().unwrap();
        assert!(matches!(
            scope,
            Scope::Course(id)
            if id == "text123"
        ));
    }

    #[test]
    fn from_str_contest() {
        let scope = "123".parse().unwrap();
        assert!(matches!(scope, Scope::Contest(123)));
    }

    #[test]
    fn to_string_course() {
        let string = Scope::Course("text123".to_owned()).to_string();
        assert_eq!(string, "text123");
    }

    #[test]
    fn to_string_contest() {
        let string = Scope::Contest(123).to_string();
        assert_eq!(string, "123");
    }
}
