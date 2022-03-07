use std::{fs, io};

use environment::Environment;

pub mod ast;
mod environment;
pub mod error;
mod evaluator;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read line failed");
    line
}

pub fn repl() {
    println!("toy lisp");

    loop {
        println!("> ");
        let text = read_line();
        match eval(&text) {
            Ok(res) => println!("{}", res),
            Err(err) => println!("{}", err.0),
        }
    }
}

pub fn run(filepath: &str) {
    let text = fs::read_to_string(filepath).expect("read file error");
    match eval(&text) {
        Ok(res) => println!("{}", res),
        Err(err) => println!("{}", err.0),
    }
}

pub fn eval(program: &str) -> Result<ast::Object, error::Error> {
    let env = &mut Environment::new_global();
    evaluator::eval_from_string(program, env)
}
