use std::fmt::Display;

#[derive(Debug)]
pub enum GreprError {
    RegexError {
        regexp_error: regex::Error,
        pattern: String,
    },

    IoError {
        file: String,
        error: std::io::Error,
    },
}

impl Display for GreprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreprError::RegexError { pattern, .. } => {
                write!(f, "Invalid pattern \"{}\"", pattern)
            }
            GreprError::IoError { file, error } => {
                write!(f, "{}: {}", file, error)
            }
        }
    }
}
