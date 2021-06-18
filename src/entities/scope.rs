use std::{num::ParseIntError, str::FromStr};

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

impl ToString for Scope {
    fn to_string(&self) -> String {
        match self {
            Scope::Course(id) => id.clone(),
            Scope::Contest(id) => id.to_string(),
        }
    }
}

// TODO: Tests
