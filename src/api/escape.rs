use std::fmt;
use std::fmt::Write;

pub struct Escape<'a>(pub &'a str);

impl<'a> fmt::Display for Escape<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &byte in self.0.as_bytes() {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                f.write_char(byte as char)?;
            } else {
                write!(f, "%{:02X}", byte)?;
            }
        }
        Ok(())
    }
}

impl<'a> From<Escape<'a>> for String {
    fn from(escape: Escape<'a>) -> Self {
        escape.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlescape() {
        let result = Escape("aBc123 +-ä").to_string();
        assert_eq!(result, "aBc123%20%2B-%C3%A4");
    }

    #[test]
    fn from_escape_to_string() {
        let result: String = String::from(Escape("aBc123 +-ä"));
        assert_eq!(result, "aBc123%20%2B-%C3%A4");
    }
}
