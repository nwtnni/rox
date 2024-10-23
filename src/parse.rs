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

    pub fn parse_literal(&mut self) -> Option<Lit> {
        let lit = match self.iter.peek()? {
            Token::Number(value) => Lit::Number(*value),
            Token::String(value) => Lit::String(value.clone()),
            Token::True => Lit::Bool(true),
            Token::False => Lit::Bool(false),
            Token::Nil => Lit::Nil,
            _ => return None,
        };

        Some(lit)
    }
}

#[derive(Debug)]
enum Expr {
    Lit(Lit),
    Unary(Unary, Box<Expr>),
    Binary(Binary, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug)]
enum Comparison {
    Less,
    LessEqual,
    Equal,
    Greater,
    GreaterEqual,
}

#[derive(Debug)]
enum Binary {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Unary {
    Negate,
    Not,
}

#[derive(Debug)]
enum Lit {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
