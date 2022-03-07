use std::{env, process};

use toy_lisp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "\
usage:

$ cargo run -- --repl

or

$ cargo run -- path-to-script-file
e.g.
$ cargo run -- example/01-add.cjs
"
        );
        process::exit(1);
    }

    let arg = args[1].as_str();

    match arg {
        "--repl" => {
            toy_lisp::repl();
        }
        _ => {
            println!("eval script file: {}", arg);
            toy_lisp::run(arg);
        }
    }
}
