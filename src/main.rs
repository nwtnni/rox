use std::env;
use std::process;

use rox::run_file;

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
