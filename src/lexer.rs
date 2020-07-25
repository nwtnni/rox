use std::iter;
use std::str;

use anyhow::anyhow;
use anyhow::Context as _;

use crate::span;
use crate::token;

/// Stateful lexer.
///
/// ```text
///
/// 0      1      2
/// +------+------+------+
/// |  'a' | '\n' |  'b' |
/// +------+------+------+
/// ^      ^
/// |      |
/// head   tail
/// ```
pub struct T<'s> {
    text: &'s str,
    head: Option<(usize, char)>,
    tail: iter::Peekable<str::CharIndices<'s>>,

    row: usize,
    col: usize,
}

impl<'s> Iterator for T<'s> {
    type Item = anyhow::Result<(span::T, token::T)>;
    fn next(&mut self) -> Option<Self::Item> {

        self.skip();

        let (_, character) = self.peek();

        match character {
        | Some(character) if is_ident_head(character) => Some(Ok(self.lex_ident())),
        | Some(character) if is_number(character) => Some(self.lex_number()),
        | Some('.') if self.peek().1.map_or(false, is_number) => Some(self.lex_number()),

        | _ => todo!(),
        }
    }
}

impl<'s> T<'s> {

    fn lex_number(&mut self) -> anyhow::Result<(span::T, token::T)> {
        let (lo, _) = self.next();

        // Only allow a single dot
        let mut dot = false;

        let hi = loop {
            match (self.peek(), dot) {
            | ((_, Some(character)), _) if is_number(character) => {
                self.next();
            }
            | ((_, Some('.')), false) => {
                dot = true;
                self.next();
            }
            | ((hi, _), _) => break hi,
            }
        };

        let span = span::T { lo, hi };
        let text = &self.text[lo.idx..hi.idx];

        text.parse::<f64>()
            .map_err(anyhow::Error::from)
            .with_context(|| anyhow!("Could not lex number at {}: '{}'", span, text))
            .map(|number| (span, token::T::Number(number)))
    }

    fn lex_ident(&mut self) -> (span::T, token::T) {
        let (lo, _) = self.next();

        let hi = loop {
            match self.peek() {
            | (_, Some(character)) if is_ident_tail(character) => {
                self.next();
            }
            | (hi, _) => break hi,
            }
        };

        let span = span::T { lo, hi };
        let token = match &self.text[lo.idx..hi.idx] {
        | "and" => token::T::And,
        | "class" => token::T::Class,
        | "else" => token::T::Else,
        | "false" => token::T::False,
        | "fun" => token::T::Fun,
        | "for" => token::T::For,
        | "if" => token::T::If,
        | "nil" => token::T::Nil,
        | "or" => token::T::Or,
        | "print" => token::T::Print,
        | "return" => token::T::Return,
        | "super" => token::T::Super,
        | "this" => token::T::This,
        | "true" => token::T::True,
        | "var" => token::T::Var,
        | "while" => token::T::While,
        | ident => token::T::Ident(String::from(ident)),
        };

        (span, token)
    }

    /// Skip past whitespace and comments.
    fn skip(&mut self) {

        #[derive(Copy, Clone)]
        enum State {
            Whitespace,
            LineComment,
            BlockComment(usize),
        }

        let mut state = State::Whitespace;

        while let (_, Some(character)) = self.peek() {
            match (state, character) {
            | (State::Whitespace, ws) if ws.is_ascii_whitespace() => {
                self.next();
            },
            | (State::Whitespace, '/') if self.peeeek() == Some('/') => {
                self.next();
                self.next();
                state = State::LineComment;
            }
            | (State::Whitespace, '/') if self.peeeek() == Some('*') => {
                self.next();
                self.next();
                state = State::BlockComment(0);
            }
            | (State::Whitespace, _) => break,
            | (State::LineComment, '\n') => {
                self.next();
                state = State::Whitespace;
            }
            | (State::LineComment, _) => {
                self.next();
            }
            | (State::BlockComment(depth), '/') if self.peeeek() == Some('*') => {
                self.next();
                self.next();
                state = State::BlockComment(depth + 1);
            }
            | (State::BlockComment(0), '*') if self.peeeek() == Some('/') => {
                self.next();
                self.next();
                state = State::Whitespace;
            }
            | (State::BlockComment(depth), '*') if self.peeeek() == Some('/') => {
                self.next();
                self.next();
                state = State::BlockComment(depth - 1);
            }
            | (State::BlockComment(_), _) => {
                self.next();
            }
            }
        }
    }

    fn peek(&mut self) -> (span::Loc, Option<char>) {
        match self.head {
        | None => (self.eof(), None),
        | Some((index, character)) => (self.loc(index), Some(character)),
        }
    }

    fn peeeek(&mut self) -> Option<char> {
        self.tail.peek().copied().map(|(_, character)| character)
    }

    fn next(&mut self) -> (span::Loc, Option<char>) {
        let (loc, character) = self.peek();
        match character {
        | None => (),
        | Some('\n') => {
            self.row += 1;
            self.col = 1;
        }
        | Some(_) => {
            self.col += 1;
        }
        }
        (loc, character)
    }

    fn loc(&self, index: usize) -> span::Loc {
        span::Loc {
            row: self.row,
            col: self.col,
            idx: index,
        }
    }

    fn eof(&self) -> span::Loc {
        span::Loc {
            row: self.row,
            col: self.col + 1,
            idx: self.text.len(),
        }
    }
}

fn is_ident_head(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_ident_tail(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_number(c: char) -> bool {
    c.is_ascii_digit()
}
