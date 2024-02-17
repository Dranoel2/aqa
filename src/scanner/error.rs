use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    UnexpectedEOF,
    UnexpectedChar(char),
    FailedToParseFloat,
    FailedToParseInt,
    Other(String),
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
            ErrorType::UnexpectedEOF => format!("Unexpected EOF"),
            ErrorType::UnexpectedChar(char) => format!("Unexpected character: '{}'", char),
            ErrorType::FailedToParseFloat => format!("failed to parse float"),
            ErrorType::FailedToParseInt => format!("failed to parse int"),
            ErrorType::Other(message) => message.to_owned(),
        };

        write!(
            f,
            "at line {}, column {}: {}",
            self.line, self.column, error_message
        )
    }
}

impl std::error::Error for Error {}
