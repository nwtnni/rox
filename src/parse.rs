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

    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.iter.peek()? {
            Token::Print => {
                self.iter.next();
                let expr = self.parse_expr().expect("Tried to print non-expression");
                Some(Stmt::Print(expr))
            }

            Token::LeftBrace => {
                self.iter.next();
                let mut stmts = Vec::new();
                while let Some(stmt) = self.parse_stmt() {
                    stmts.push(stmt);
                    match self.iter.next() {
                        Some(Token::Semicolon) => {}
                        _ => panic!("Expected semicolon after statement"),
                    }
                }

                match self.iter.next() {
                    Some(Token::RightBrace) => {}
                    _ => panic!("Expeceted closing brace after sequence!"),
                }

                Some(Stmt::Seq(stmts))
            }

            Token::Var => {
                self.iter.next();

                let Some(Token::Identifier(var)) = self.iter.next() else {
                    panic!("Expected identifier")
                };

                if !matches!(self.iter.next(), Some(Token::Equal)) {
                    panic!("Expected =")
                }

                let expr = self.parse_expr().expect("No expression found");
                Some(Stmt::Assign {
                    lhs: var.clone(),
                    rhs: expr,
                })
            }

            _ => None,
        }
    }

    pub fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_binary(0)
    }

    // literal * literal * literal
    pub fn parse_binary(&mut self, min: u8) -> Option<Expr> {
        let mut expr = self.parse_unary()?;

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
        let Some(op) = self
            .iter
            .next_if(|token| matches!(token, Token::Bang | Token::Minus))
        else {
            return self.parse_literal().map(Expr::Lit);
        };

        let inner = self.parse_unary().expect("Expected expression");
        let op = match op {
            Token::Bang => Unary::Not,
            Token::Minus => Unary::Negate,
            _ => unreachable!(),
        };

        Some(Expr::Unary(op, Box::new(inner)))
    }

    pub fn parse_literal(&mut self) -> Option<Lit> {
        let lit = match self.iter.peek()? {
            Token::Identifier(ident) => Lit::Var(ident.clone()),
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
pub enum Stmt {
    Print(Expr),
    Seq(Vec<Stmt>),
    Assign { lhs: String, rhs: Expr },
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

#[derive(Clone, Debug)]
pub enum Lit {
    Var(String),
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
