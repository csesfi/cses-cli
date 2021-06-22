use std::{fmt, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Course(String),
    Contest(u64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskId {
    Number(u64),
    Letter(char),
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

impl FromStr for TaskId {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.len() == 1 {
            if let Some(letter) = s.chars().next() {
                if letter.is_ascii_alphabetic() {
                    return Ok(TaskId::Letter(letter.to_ascii_uppercase()));
                }
            }
        }
        Ok(TaskId::Number(s.parse()?))
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

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskId::Number(id) => write!(f, "{}", id)?,
            TaskId::Letter(id) => write!(f, "{}", id)?,
        }
        Ok(())
    }
}

// TODO: Tests
