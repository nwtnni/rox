use core::iter;

//  private static void run(String source) {
//    Scanner scanner = new Scanner(source);
//    List<Token> tokens = scanner.scanTokens();
//
//    // For now, just print the tokens.
//    for (Token token : tokens) {
//      System.out.println(token);
//    }
//  }
pub struct Scanner<'source> {
    iter: iter::Peekable<std::str::CharIndices<'source>>,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            iter: source.char_indices().peekable(),
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Option::Some((_, char)) = self.iter.next() {
            let token = match char {
                ' ' | '\n' | '\t' => continue,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                ',' => Token::Comma,
                '-' => Token::Minus,
                '+' => Token::Plus,
                ';' => Token::Semicolon,
                '/' => Token::Slash,
                '*' => Token::Star,

                '!' => self.scan_double('=', Token::Bang, Token::BangEqual),
                '>' => self.scan_double('=', Token::Greater, Token::GreaterEqual),
                '<' => self.scan_double('=', Token::Less, Token::LessEqual),
                '=' => self.scan_double('=', Token::Equal, Token::EqualEqual),

                n @ ('0'..='9' | '.') => {
                    let mut nums = String::new();
                    nums.push(n);
                    while let Option::Some((_, char)) = self.iter.peek().copied() {
                        match char {
                            n2 @ ('0'..='9' | '.') => {
                                self.iter.next();
                                nums.push(n2);
                            }
                            _ => break,
                        }
                    }

                    if nums == "." {
                        Token::Dot
                    } else {
                        Token::Number(nums.parse().unwrap())
                    }
                }

                '\"' => {
                    let mut buffer = String::new();
                    while let Option::Some((_, char)) = self.iter.next() {
                        if char != '\"' {
                            buffer.push(char)
                        } else {
                            break;
                        }
                    }
                    Token::String(buffer)
                }

                n @ ('a'..='z' | '_') => {
                    let mut str = String::new();
                    str.push(n);
                    while let Option::Some((_, char)) = self.iter.peek().copied() {
                        match char {
                            'a'..='z' | '0'..='9' | '_' => {
                                self.iter.next();
                                str.push(char);
                            }

                            _ => break,
                        }
                    }

                    match str.as_str() {
                        "and" => Token::And,
                        "class" => Token::Class,
                        "else" => Token::Else,
                        "false" => Token::False,
                        "fun" => Token::Fun,
                        "for" => Token::For,
                        "if" => Token::If,
                        "nil" => Token::Nil,
                        "or" => Token::Or,
                        "print" => Token::Print,
                        "return" => Token::Return,
                        "super" => Token::Super,
                        "this" => Token::This,
                        "true" => Token::True,
                        "var" => Token::Var,
                        "while" => Token::While,
                        _ => Token::Identifier(str),
                    }
                }

                char => panic!("Unrecognized character: {char}"),
            };

            tokens.push(token);
        }
        tokens
    }

    fn scan_double(&mut self, next: char, single: Token, double: Token) -> Token {
        if let Option::Some((_, char)) = self.iter.next() {
            if char == next {
                double
            } else {
                single
            }
        } else {
            single
        }
    }
}

pub(crate) fn run(source: String) -> Vec<Token> {
    let mut scanner = Scanner::new(&source);
    scanner.scan()
}

// class Token {
//   final TokenType type;
//   final String lexeme;
//   final Object literal;
//   final int line;
//
//   Token(TokenType type, String lexeme, Object literal, int line) {
//     this.type = type;
//     this.lexeme = lexeme;
//     this.literal = literal;
//     this.line = line;
//   }
//
//   public String toString() {
//     return type + " " + lexeme + " " + literal;
//   }
// }
#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier(String),
    String(String),
    Number(f64),

    // keywords.
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

    Eof,
}

impl Token {
    pub(crate) fn precedence(&self) -> Option<u8> {
        let precedence = match self {
            Token::Star | Token::Slash => 4,
            Token::Minus | Token::Plus => 3,
            Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => 2,
            Token::EqualEqual | Token::BangEqual => 1,
            _ => return None,
        };

        Some(precedence)
    }
}
