use std::fmt;

use crate::{scanner::Token, Position};

#[derive(Debug)]
pub enum ErrorType {
    UnexpectedToken(Token),
    ExpectedRightParen,
    Other(String),
}

#[derive(Debug)]
pub struct Error {
    position: Position,
    pub error_type: ErrorType,
}

impl Error {
    pub fn new(position: Position, error_type: ErrorType) -> Self {
        Self {
            position,
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
            ErrorType::ExpectedRightParen => String::from("Expected Right Parenthesis"),
            ErrorType::Other(message) => message.to_owned(),
        };

        write!(
            f,
            "at line {}, column {}: {}",
            self.position.line, self.position.column, error_message
        )
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
