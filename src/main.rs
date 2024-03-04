use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

mod interpreter;
mod parser;
mod scanner;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    line: usize,
    column: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

pub fn run(contents: String) -> anyhow::Result<()> {
    let mut scanner = scanner::Scanner::new(contents);

    let mut tokens = Vec::new();

    let mut exit = false;
    while !exit {
        let token = scanner.scan_token()?;
        if token.token_type == scanner::TokenType::EOF {
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

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let path = &args[1];
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        run(contents)?;
    } else {
        println!("Usage: aqa-interpreter <file>");
    }
    Ok(())
}
