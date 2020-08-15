use std::convert;

use crate::span;
use crate::token;

#[derive(thiserror::Error)]
#[derive(Clone, Debug)]
pub enum Error {
    #[error("Expected binary op token, but found {:?} token at {}", _1, _0)]
    NotBop(span::T, token::T),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Exp {
    Bin(Bop, Box<Exp>, Box<Exp>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Bop {
    Gt,
    Ge,
    Le,
    Lt,
    Ne,
    Eq,

    Add,
    Sub,
    Mul,
    Div,
}

pub enum Uop {


}

impl convert::TryFrom<(span::T, token::T)> for Bop {
    type Error = Error;
    fn try_from((span, token): (span::T, token::T)) -> Result<Self, Self::Error> {
        match token {
        | token::T::Lt => Ok(Bop::Lt),
        | token::T::Le => Ok(Bop::Le),
        | token::T::Ge => Ok(Bop::Ge),
        | token::T::Gt => Ok(Bop::Gt),
        | token::T::Ne => Ok(Bop::Ne),
        | token::T::Eq => Ok(Bop::Eq),
        | token => Err(Error::NotBop(span, token)),
        }
    }
}
