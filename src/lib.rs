pub mod ast;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod token;

#[derive(thiserror::Error)]
#[derive(Clone, Debug)]
pub enum Error {
    #[error("Internal AST construction error")]
    Ast(#[from] ast::Error),

    #[error("Lexer error")]
    Lexer(#[from] lexer::Error),

    #[error("Parser error")]
    Parser(#[from] parser::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;
