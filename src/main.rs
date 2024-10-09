use std::env;
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
    run(source);
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
struct Scanner {
    source: String,
}
impl Scanner {
    fn scan_tokens(self) -> Vec<Token> {
        let mut iter = self.source.char_indices().peekable();
        let mut tokens = Vec::new();
        while let Option::Some((byte, char)) = iter.next() {
            match char {
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '>' => match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        tokens.push(Token::GreaterEqual);
                    }
                    Some(_) | None => tokens.push(Token::Greater),
                },

                _ => {}
            }
        }
        tokens
    }
}

fn run(source: String) {
    let scanner = Scanner { source };
    let tokens = scanner.scan_tokens();
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
