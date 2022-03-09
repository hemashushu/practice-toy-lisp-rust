use std::{fs, io, rc::Rc, cell::RefCell};

use env::Environment;

use crate::error::Error;

mod token;
mod parser;
pub mod ast;
pub mod env;
pub mod error;
pub mod eval;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read line failed");
    line
}

pub fn repl() {
    println!("toy lisp");

    let env = Environment::new_global();
    let rc_env = env.to_rc_env(); // Rc::new(RefCell::new(Some(env)));

    loop {
        println!("> ");
        let text = read_line();
        match eval_program(&text, &rc_env) {
            Ok(res) => println!("{}", res),
            Err(err) => match err {
                Error::EvalError(msg) => println!("{}", msg),
            },
        }
    }
}

pub fn run(filepath: &str) {
    let text = fs::read_to_string(filepath).expect("read file error");

    let env = Environment::new_global();
    let rc_env = env.to_rc_env(); // Rc::new(RefCell::new(Some(env)));

    match eval_program(&text, &rc_env) {
        Ok(res) => println!("{}", res),
        Err(err) => match err {
            Error::EvalError(msg) => println!("{}", msg),
        },
    }
}

fn eval_program(
    program: &str,
    rc_env: &Rc<RefCell<Option<Environment>>>,
) -> Result<ast::Object, error::Error> {

    eval::eval_from_string(program, rc_env)
}
