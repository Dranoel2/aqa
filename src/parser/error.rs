use std::fmt;

use crate::scanner::Token;

#[derive(Debug)]
pub enum ErrorType {
    UnexpectedToken(Token),
    ExpectedLeftParen,
    Other(String),
}

#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub column: usize,
    pub error_type: ErrorType,
}

impl Error {
    pub fn new(token: &Token, error_type: ErrorType) -> Self {
        Self {
            line: token.line,
            column: token.column,
            error_type,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match &self.error_type {
            ErrorType::UnexpectedToken(token) => {
                format!("Unexpected token: '{:?}'", token.token_type)
            }
            ErrorType::ExpectedLeftParen => String::from("Expected Left Parenthesis"),
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

pub type Result<T> = std::result::Result<T, Error>;
