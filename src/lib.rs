use interpret::Interpreter;

pub mod interpret;
pub mod lex;
pub mod parse;

// private static void runFile(String path) throws IOException {
//    byte[] bytes = Files.readAllBytes(Paths.get(path));
//    run(new String(bytes, Charset.defaultCharset()));
// }
pub fn run_file(path: String) {
    let source = std::fs::read_to_string(&path).unwrap();
    let tokens = lex::run(source);
    let mut parser = parse::Parser::new(&tokens);
    let ast = parser.parse_stmt().expect("Failed to parse statement");
    let mut interpreter = Interpreter::new();
    interpreter.eval_stmt(&ast);
}
