mod error;
mod interpreter;
mod parser;
mod scanner;

pub use error::*;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    line: usize,
    column: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

pub fn run(contents: String) -> Result<Value> {
    let mut scanner = scanner::Scanner::new(contents);

    let mut tokens = Vec::new();

    let mut exit = false;
    while !exit {
        let token = scanner.scan_token()?;
        if token.token_type == scanner::TokenType::Eof {
            exit = true;
        }
        tokens.push(token);
    }

    println!("{:?}", tokens);

    let mut parser = parser::Parser::new(tokens);

    let expression = parser.parse()?;

    println!("{:?}", expression);

    let value = interpreter::evaluate(expression)?;

    println!("{:?}", value);

    Ok(value)
}
