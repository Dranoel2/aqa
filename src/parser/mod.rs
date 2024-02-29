use crate::scanner::{Token, TokenType};

mod error;

pub use error::*;

use std::mem;

#[derive(Debug)]
pub enum Expr {
    Literal(Token),
    Bool(bool),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.index].clone()
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.index == self.tokens.len() {
            false
        } else {
            mem::discriminant(&self.peek().token_type) == mem::discriminant(&token_type)
        }
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    fn advance(&mut self) -> Token {
        if self.index != self.tokens.len() {
            self.index += 1;
        }
        self.previous()
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token(TokenType::EqualTo) || self.match_token(TokenType::NotEqualTo) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.match_token(TokenType::LessThan)
            || self.match_token(TokenType::LessThanOrEqualTo)
            || self.match_token(TokenType::GreaterThan)
            || self.match_token(TokenType::GreaterThanOrEqualTo)
        {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.match_token(TokenType::Subtract) || self.match_token(TokenType::Add) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.match_token(TokenType::Multiply) || self.match_token(TokenType::Divide) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_token(TokenType::LogicalNot) || self.match_token(TokenType::LogicalAnd) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(operator, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        let expr = match self.peek().token_type {
            TokenType::Bool(_) | TokenType::Int(_) | TokenType::Float(_) | TokenType::String(_) => {
                Expr::Literal(self.advance())
            }

            TokenType::LeftParen => {
                self.advance();
                if !self.match_token(TokenType::LeftParen) {
                    return Err(Error::new(&self.peek(), ErrorType::ExpectedLeftParen));
                } else {
                    self.expression()?
                }
            }

            _ => {
                let token = self.peek();
                return Err(Error::new(
                    &token,
                    ErrorType::UnexpectedToken(token.clone()),
                ));
            }
        };

        Ok(expr)
    }
}
