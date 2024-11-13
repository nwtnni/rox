use core::iter;
use core::slice;

use crate::lex::Token;

pub struct Parser<'source> {
    iter: iter::Peekable<slice::Iter<'source, Token>>,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: &'source [Token]) -> Self {
        Self {
            iter: tokens.iter().peekable(),
        }
    }

    // literal * literal * literal
    pub fn parse_binary(&mut self, min: u8) -> Option<Expr> {
        let mut expr = Expr::Lit(self.parse_literal()?);

        while let Some(pow) = self.iter.peek().copied().and_then(Token::precedence) {
            if pow * 2 < min {
                break;
            }

            let token = self.iter.next();
            let rhs = self
                .parse_binary(pow * 2 + 1)
                .expect("No literal after binary operator");

            let op = match token {
                Some(Token::Star) => Binary::Mul,
                Some(Token::Slash) => Binary::Div,
                Some(Token::Plus) => Binary::Add,
                Some(Token::Minus) => Binary::Sub,
                _ => unreachable!(),
            };

            expr = Expr::Binary(op, Box::new(expr), Box::new(rhs));
        }

        Some(expr)
    }

    pub fn parse_unary(&mut self) -> Option<Expr> {
        let op = self
            .iter
            .next_if(|token| matches!(token, Token::Bang | Token::Minus))?;

        let inner = self.parse_literal().expect("Expected expression");
        let op = match op {
            Token::Bang => Unary::Not,
            Token::Minus => Unary::Negate,
            _ => unreachable!(),
        };

        Some(Expr::Unary(op, Box::new(Expr::Lit(inner))))
    }

    pub fn parse_literal(&mut self) -> Option<Lit> {
        let lit = match self.iter.peek()? {
            Token::Number(value) => Lit::Number(*value),
            Token::String(value) => Lit::String(value.clone()),
            Token::True => Lit::Bool(true),
            Token::False => Lit::Bool(false),
            Token::Nil => Lit::Nil,
            _ => return None,
        };

        self.iter.next();
        Some(lit)
    }
}

#[derive(Debug)]
pub enum Expr {
    Lit(Lit),
    Unary(Unary, Box<Expr>),
    Binary(Binary, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Comparison {
    Less,
    LessEqual,
    Equal,
    Greater,
    GreaterEqual,
}

#[derive(Debug)]
pub enum Binary {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Unary {
    Negate,
    Not,
}

#[derive(Debug)]
pub enum Lit {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
