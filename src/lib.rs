pub mod lex;

// private static void runFile(String path) throws IOException {
//    byte[] bytes = Files.readAllBytes(Paths.get(path));
//    run(new String(bytes, Charset.defaultCharset()));
// }
pub fn run_file(path: String) {
    let source = std::fs::read_to_string(&path).unwrap();
    lex::run(source)
}
