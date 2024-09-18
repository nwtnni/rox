use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[0].clone());
    } else {
        todo!()
    }
}

fn run_file(path: String) {
    panic!("Run {path}");
}
