use std::{fmt, num::ParseIntError, str::FromStr};

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
