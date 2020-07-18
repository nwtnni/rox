#[derive(Clone, Debug, PartialEq, Eq)]
pub enum T {
    Ident(String),
    String(String),
    Number(i64),

    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Not,
    Ne,
    Eq,
    EqEq,
    Gt,
    Ge,
    Lt,
    Le,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}
