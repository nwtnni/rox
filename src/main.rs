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
        todo!()
    }
}

#[derive(Debug)]
struct Token;

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
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}
