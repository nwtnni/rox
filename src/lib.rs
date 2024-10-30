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
    dbg!(parser.parse_factor());
}
