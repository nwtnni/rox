use std::convert::TryFrom as _;
use std::iter;

use crate::ast;
use crate::lexer;
use crate::token;

pub struct T<'s> {
    tokens: iter::Peekable<lexer::T<'s>>,
}

#[derive(thiserror::Error)]
#[derive(Clone, Debug)]
pub enum Error {

}

impl<'s> T<'s> {
    fn parse_exp(&mut self) -> crate::Result<ast::Exp> {
        todo!();
    }

    fn parse_equality(&mut self) -> crate::Result<ast::Exp> {
        let mut lhs = todo!();
        loop {
            match self.tokens.peek().cloned().transpose()? {
            | Some((span, token @ token::T::Ne))
            | Some((span, token @ token::T::Eq)) => {
                self.tokens.next();
                let bop = ast::Bop::try_from((span, token))?;
                let rhs = todo!();
                lhs = ast::Exp::Bin(bop, Box::new(lhs), Box::new(rhs));
            }
            | _ => return Ok(lhs),
            }
        }
    }
}
