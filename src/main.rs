use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    process,
    str::Chars,
};

#[derive(Debug)]
struct Token {
    token_type: TokenType,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Identifier(String),
    String(String),

    Assign,
    Constant,

    Add,
    Subtract,
    Multiply,
    Divide,
    IntegerDivide,
    Modulus,

    LessThan,
    GreaterThan,
    EqualTo,
    NotEqualTo,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,

    LogicalAnd,
    LogicalOr,
    LogicalNot,

    Repeat,
    Until,
    While,
    EndWhile,

    For,
    To,
    In,
    EndFor,

    If,
    Then,
    Else,
    EndIf,

    Output,

    LineBreak,
    EOF,
}

struct Scanner {}

impl Scanner {
    fn token(token_type: TokenType) -> io::Result<Token> {
        Ok(Token { token_type })
    }

    fn scan_token(contents: &mut Chars) -> io::Result<Token> {
        let mut peekable = contents.peekable();

        if let Some(first_char) = peekable.next() {
            let mut char = first_char;

            let mut is_end = false;
            while char.is_whitespace() && !is_end {
                if let Some(next_char) = peekable.next() {
                    char = next_char;
                } else {
                    is_end = true;
                }
            }

            if is_end {
                Self::token(TokenType::EOF)
            } else {
                match char {
                    '<' => {
                        if let Some(next) = peekable.peek() {
                            match next {
                                '-' => {
                                    peekable.next();
                                    Self::token(TokenType::Assign)
                                }
                                '=' => {
                                    peekable.next();
                                    Self::token(TokenType::LessThanOrEqualTo)
                                }
                                _ => Self::token(TokenType::LessThan),
                            }
                        } else {
                            Self::token(TokenType::LessThan)
                        }
                    }

                    '>' => {
                        if let Some(next) = peekable.peek() {
                            match next {
                                '=' => {
                                    peekable.next();
                                    Self::token(TokenType::GreaterThanOrEqualTo)
                                }
                                _ => Self::token(TokenType::GreaterThan),
                            }
                        } else {
                            Self::token(TokenType::GreaterThan)
                        }
                    }

                    '!' => {
                        if let Some(next) = peekable.peek() {
                            match next {
                                '=' => Self::token(TokenType::NotEqualTo),
                                _ => Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("Unexpected character: {}", next.to_string()),
                                )),
                            }
                        } else {
                            Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected EOF"))
                        }
                    }

                    '+' => Self::token(TokenType::Add),
                    '-' => Self::token(TokenType::Subtract),
                    '*' => Self::token(TokenType::Multiply),
                    '/' => Self::token(TokenType::Divide),

                    '=' => Self::token(TokenType::EqualTo),

                    '\n' => Self::token(TokenType::LineBreak),

                    _ => Self::token(TokenType::String(char.to_string())),
                }
            }
        } else {
            Self::token(TokenType::EOF)
        }
    }

    fn scan(contents: &mut Chars) -> io::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        let mut exit = false;
        while !exit {
            let token = Self::scan_token(contents)?;
            if token.token_type == TokenType::EOF {
                exit = true;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let path = &args[1];
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let tokens = Scanner::scan(&mut contents.chars())?;

        println!("{:?}", tokens);
    } else {
        println!("Usage: aqa-interpreter <file>");
        process::exit(-1);
    }

    Ok(())
}
