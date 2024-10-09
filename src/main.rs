use std::env;
use std::iter;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        todo!()
    }
}

// private static void runFile(String path) throws IOException {
//    byte[] bytes = Files.readAllBytes(Paths.get(path));
//    run(new String(bytes, Charset.defaultCharset()));
// }
fn run_file(path: String) {
    let source = std::fs::read_to_string(&path).unwrap();
    run(source)
}

//  private static void run(String source) {
//    Scanner scanner = new Scanner(source);
//    List<Token> tokens = scanner.scanTokens();
//
//    // For now, just print the tokens.
//    for (Token token : tokens) {
//      System.out.println(token);
//    }
//  }
struct Scanner<'source> {
    iter: iter::Peekable<std::str::CharIndices<'source>>,
}

impl<'source> Scanner<'source> {
    fn new(source: &'source str) -> Self {
        Self {
            iter: source.char_indices().peekable(),
        }
    }

    fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Option::Some((_, char)) = self.iter.next() {
            let token = match char {
                ' ' | '\n' | '\t' => continue,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
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

                _ => todo!(),
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

fn run(source: String) {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan();
    for token in tokens {
        println!("{token:?}");
    }
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
#[derive(Debug)]
#[allow(unused)]
enum Token {
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
