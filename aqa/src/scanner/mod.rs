mod error;
pub use error::*;

use crate::{Position, Value};

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Literal(Value),

    Identifier(String),

    Assign,
    Constant,

    Add,
    Subtract,
    Multiply,
    Divide,
    IntDivide,
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
    Eof,

    LeftParen,
    RightParen,
}

pub struct Scanner {
    contents: String,
    index: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(contents: String) -> Self {
        Self {
            index: 0,
            line: 1,
            column: 1,
            contents,
        }
    }

    fn resolve_word(word: String) -> TokenType {
        match word.as_str() {
            "CONSTANT" => TokenType::Constant,

            "DIV" => TokenType::IntDivide,
            "MOD" => TokenType::Modulus,

            "AND" => TokenType::LogicalAnd,
            "OR" => TokenType::LogicalOr,
            "NOT" => TokenType::LogicalNot,

            "REPEAT" => TokenType::Repeat,
            "UNTIL" => TokenType::Until,
            "WHILE" => TokenType::While,
            "ENDWHILE" => TokenType::EndWhile,

            "FOR" => TokenType::For,
            "TO" => TokenType::To,
            "IN" => TokenType::In,
            "ENDFOR" => TokenType::EndFor,

            "IF" => TokenType::If,
            "THEN" => TokenType::Then,
            "ELSE" => TokenType::Else,
            "ENDIF" => TokenType::EndIf,

            "OUTPUT" => TokenType::Output,

            "True" => TokenType::Literal(Value::Bool(true)),
            "False" => TokenType::Literal(Value::Bool(false)),

            _ => TokenType::Identifier(word),
        }
    }

    fn next(&mut self) -> Option<char> {
        let result = self.contents.chars().nth(self.index);
        self.index += 1;
        if let Some(char) = result {
            if char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        result
    }

    fn peek(&mut self) -> Option<char> {
        self.contents.chars().nth(self.index)
    }

    pub fn scan_token(&mut self) -> Result<Token> {
        macro_rules! error_value {
            ($error_type:expr) => {
                Err(Error {
                    line: self.line,
                    column: self.column,
                    error_type: $error_type,
                })
            };
        }

        macro_rules! error {
            ($error_type:ident) => {
                error_value!(ErrorType::$error_type)
            };
            ($error_type:ident($value:expr)) => {
                error_value!(ErrorType::$error_type($value))
            };
        }

        let mut starting_column = self.column;

        macro_rules! token_value {
            ($token_type:expr) => {
                Ok(Token {
                    token_type: $token_type,
                    position: Position {
                        line: self.line,
                        column: starting_column,
                    },
                })
            };
        }
        macro_rules! token {
            ($token_type:ident) => {
                token_value!(TokenType::$token_type)
            };
            ($token_type:ident($value:expr)) => {
                token_value!(TokenType::$token_type($value))
            };
        }

        if let Some(char) = self.next() {
            let mut char = char;

            let mut is_end = false;
            while char.is_ascii_whitespace() && !is_end {
                if let Some(next_char) = self.next() {
                    char = next_char;
                } else {
                    is_end = true;
                }
            }

            starting_column = self.column - 1;

            if is_end {
                token!(Eof)
            } else {
                match char {
                    '<' => {
                        if let Some(next) = self.peek() {
                            match next {
                                '-' => {
                                    self.index += 1;
                                    token!(Assign)
                                }
                                '=' => {
                                    self.index += 1;
                                    token!(LessThanOrEqualTo)
                                }
                                _ => token!(LessThan),
                            }
                        } else {
                            token!(LessThan)
                        }
                    }

                    '>' => {
                        if let Some(next) = self.peek() {
                            match next {
                                '=' => {
                                    self.index += 1;
                                    token!(GreaterThanOrEqualTo)
                                }
                                _ => token!(GreaterThan),
                            }
                        } else {
                            token!(GreaterThan)
                        }
                    }

                    '!' => {
                        if let Some(next) = self.peek() {
                            match next {
                                '=' => {
                                    self.index += 1;
                                    token!(NotEqualTo)
                                }
                                _ => error!(UnexpectedChar(next)),
                            }
                        } else {
                            error!(UnexpectedEOF)
                        }
                    }

                    '+' => token!(Add),
                    '-' => token!(Subtract),
                    '*' => token!(Multiply),
                    '/' => token!(Divide),

                    '=' => token!(EqualTo),

                    '\n' => token!(LineBreak),

                    '(' => token!(LeftParen),
                    ')' => token!(RightParen),

                    '\'' => {
                        let mut string = String::new();

                        let mut is_end = false;
                        let mut escape = false;
                        while let Some(char) = self.next() {
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
                            token!(Literal(Value::String(string)))
                        } else {
                            error!(UnexpectedEOF)
                        }
                    }

                    _ if char.is_ascii_digit() => {
                        let mut string = String::new();
                        string.push(char);

                        let mut is_float = false;
                        while let Some(char) = self.peek() {
                            if char.is_numeric() {
                                self.next();
                                string.push(char);
                            } else if char == '.' {
                                self.next();
                                is_float = true;
                                string.push(char);
                            } else if !char.is_alphabetic() {
                                break;
                            } else {
                                return error!(UnexpectedChar(char));
                            }
                        }

                        if is_float {
                            let parse_result = string.parse::<f64>();
                            if let Ok(result) = parse_result {
                                token!(Literal(Value::Float(result)))
                            } else {
                                error!(FailedToParseFloat)
                            }
                        } else {
                            let parse_result = string.parse::<i64>();
                            if let Ok(result) = parse_result {
                                token!(Literal(Value::Int(result)))
                            } else {
                                error!(FailedToParseInt)
                            }
                        }
                    }

                    _ if char.is_alphabetic() => {
                        let mut string = String::new();
                        string.push(char);

                        while let Some(char) = self.next() {
                            if char.is_alphanumeric() {
                                string.push(char);
                            } else {
                                break;
                            }
                        }

                        token_value!(Self::resolve_word(string))
                    }

                    _ => error!(UnexpectedChar(char)),
                }
            }
        } else {
            token!(Eof)
        }
    }
}
