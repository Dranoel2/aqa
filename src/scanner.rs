use std::{io, str::Chars};

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier(String),
    String(String),
    Int(i64),
    Float(f64),

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

macro_rules! token {
    ($token_type:ident) => {
        Ok(Token {
            token_type: TokenType::$token_type,
        })
    };
    ($token_type:ident($value:expr)) => {
        Ok(Token {
            token_type: TokenType::$token_type($value),
        })
    };
}

macro_rules! raw_error {
    ($($t:tt)*) => {
        io::Error::new(io::ErrorKind::InvalidData, format!($($t)*))
    };
}

macro_rules! error {
    ($($t:tt)*) => {
        Err(raw_error!($($t)*))
    };
}

pub fn scan_token(contents: &mut Chars) -> io::Result<Token> {
    let mut peekable = contents.peekable();

    if let Some(first_char) = peekable.next() {
        let mut char = first_char;

        let mut is_end = false;
        while char.is_ascii_whitespace() && !is_end {
            if let Some(next_char) = peekable.next() {
                char = next_char;
            } else {
                is_end = true;
            }
        }

        if is_end {
            token!(EOF)
        } else {
            match char {
                '<' => {
                    if let Some(next) = peekable.peek() {
                        match next {
                            '-' => {
                                peekable.next();
                                token!(Assign)
                            }
                            '=' => {
                                peekable.next();
                                token!(LessThanOrEqualTo)
                            }
                            _ => token!(LessThan),
                        }
                    } else {
                        token!(LessThan)
                    }
                }

                '>' => {
                    if let Some(next) = peekable.peek() {
                        match next {
                            '=' => {
                                peekable.next();
                                token!(GreaterThanOrEqualTo)
                            }
                            _ => token!(GreaterThan),
                        }
                    } else {
                        token!(GreaterThan)
                    }
                }

                '!' => {
                    if let Some(next) = peekable.peek() {
                        match next {
                            '=' => token!(NotEqualTo),
                            _ => error!("Unexpected character: {}", next.to_string()),
                        }
                    } else {
                        error!("Unexpected EOF")
                    }
                }

                '+' => token!(Add),
                '-' => token!(Subtract),
                '*' => token!(Multiply),
                '/' => token!(Divide),

                '=' => token!(EqualTo),

                '\n' => token!(LineBreak),

                '\'' => {
                    let mut string = String::new();

                    let mut is_end = false;
                    let mut escape = false;
                    while let Some(char) = peekable.next() {
                        match char {
                            '\\' if !escape => {
                                escape = true;
                            }
                            '\'' if !escape => {
                                is_end = true;
                                break;
                            }
                            'n' if escape => {
                                string.push('\n');
                            }
                            _ => {
                                string.push(char);
                            }
                        }
                    }

                    if is_end {
                        token!(String(string))
                    } else {
                        error!("Unexpected EOF")
                    }
                }

                _ if char.is_ascii_digit() => {
                    let mut string = String::new();
                    string.push(char);

                    let mut is_float = false;
                    let mut error: Option<io::Error> = None;
                    while let Some(char) = peekable.next() {
                        if char.is_numeric() {
                            string.push(char);
                        } else if char == '.' {
                            is_float = true;
                            string.push(char);
                        } else if char.is_ascii_whitespace() {
                            break;
                        } else {
                            error = Some(raw_error!("Unexpected character: {}", char));
                            break;
                        }
                    }

                    if let Some(error) = error {
                        Err(error)
                    } else {
                        if is_float {
                            let parse_result = string.parse::<f64>();
                            if let Ok(result) = parse_result {
                                token!(Float(result))
                            } else {
                                error!("Failed to parse float")
                            }
                        } else {
                            let parse_result = string.parse::<i64>();
                            if let Ok(result) = parse_result {
                                token!(Int(result))
                            } else {
                                error!("Failed to parse int")
                            }
                        }
                    }
                }

                _ if char.is_alphabetic() => {
                    let mut string = String::new();
                    string.push(char);

                    while let Some(char) = peekable.next() {
                        if char.is_alphanumeric() {
                            string.push(char);
                        } else {
                            break;
                        }
                    }

                    token!(Identifier(string))
                }

                _ => error!("Unexpected character: {}", char),
            }
        }
    } else {
        token!(EOF)
    }
}

pub fn scan(contents: &mut Chars) -> io::Result<Vec<Token>> {
    let mut tokens = Vec::new();

    let mut exit = false;
    while !exit {
        let token = scan_token(contents)?;
        if token.token_type == TokenType::EOF {
            exit = true;
        }
        tokens.push(token);
    }

    Ok(tokens)
}
