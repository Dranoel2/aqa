use core::fmt;

use crate::{interpreter, parser, scanner};

#[derive(Debug)]
pub enum Error {
    ScannerError(scanner::Error),
    ParserError(parser::Error),
    InterpreterError(interpreter::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ScannerError(error) => error.fmt(f),
            Error::ParserError(error) => error.fmt(f),
            Error::InterpreterError(error) => error.fmt(f),
        }
    }
}

impl From<scanner::Error> for Error {
    fn from(value: scanner::Error) -> Self {
        Self::ScannerError(value)
    }
}

impl From<parser::Error> for Error {
    fn from(value: parser::Error) -> Self {
        Self::ParserError(value)
    }
}

impl From<interpreter::Error> for Error {
    fn from(value: interpreter::Error) -> Self {
        Self::InterpreterError(value)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
