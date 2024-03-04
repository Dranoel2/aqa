use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    UnexpectedEOF,
    UnexpectedChar(char),
    FailedToParseFloat,
    FailedToParseInt,
}

#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub column: usize,
    pub error_type: ErrorType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match &self.error_type {
            ErrorType::UnexpectedEOF => "Unexpected EOF".to_string(),
            ErrorType::UnexpectedChar(char) => format!("Unexpected character: '{}'", char),
            ErrorType::FailedToParseFloat => "failed to parse float".to_string(),
            ErrorType::FailedToParseInt => "failed to parse int".to_string(),
        };

        write!(
            f,
            "at line {}, column {}: {}",
            self.line, self.column, error_message
        )
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
