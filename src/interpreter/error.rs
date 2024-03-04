use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    MismatchedType,
    Other(String),
}

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
}

impl Error {
    pub fn new(error_type: ErrorType) -> Self {
        Self { error_type }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match &self.error_type {
            ErrorType::MismatchedType => String::from("Mismatched Type"),
            ErrorType::Other(message) => message.to_owned(),
        };

        write!(f, "{}", error_message)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
