use std::fs;

use rox::lex::Scanner;

#[test]
fn lex() {
    insta::glob!("lex/*.rox", |path| {
        let source = fs::read_to_string(path).unwrap();
        let mut scanner = Scanner::new(&source);
        let tokens = scanner.scan();
        insta::assert_debug_snapshot!(tokens);
    });
}
