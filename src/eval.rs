use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{Func, Object};
use crate::env::{rc_env_define, rc_env_lookup, Environment};
use crate::error::Error;
use crate::parser::parse;
use crate::token::tokenize;

fn eval(node: &Object, rc_env: &Rc<RefCell<Option<Environment>>>) -> Result<Object, Error> {
    match node {
        // 标识符，从 Environment 里获取对应的值
        // 注：lookup 方法返回的是值的 clone
        Object::Symbol(name) => match rc_env_lookup(rc_env, name) {
            Some(obj) => Ok(obj),
            None => Err(Error::EvalError(format!("identifier not found: {}", name))),
        },
        // 数字
        Object::Number(_) => Ok(node.clone()),
        // 布尔值
        Object::Bool(_) => Ok(node.clone()),
        // 列表
        Object::List(list) => {
            let (first_node, rest_nodes) = list
                .split_first()
                .ok_or(Error::EvalError("empty list".to_string()))?;
            eval_list(first_node, rest_nodes, rc_env)
        }
        _ => Err(Error::EvalError("unsupported object".to_string())),
    }
}

fn eval_list(
    node: &Object,
    rest_nodes: &[Object],
    rc_env: &Rc<RefCell<Option<Environment>>>,
) -> Result<Object, Error> {
    match node {
        Object::Symbol(name) => {
            // 先判断是否关键字，比如 do, let, if, defn, fn 等
            match name.as_str() {
                "do" => eval_do(rest_nodes, rc_env),
                "let" => eval_let(rest_nodes, rc_env),
                "if" => eval_if(rest_nodes, rc_env),
                "defn" => eval_defn(rest_nodes, rc_env),
                "fn" => eval_fn(rest_nodes, rc_env),
                _ => {
                    // 预期是函数（内置函数、用户自定义函数或者匿名函数）
                    eval_function_call(node, rest_nodes, rc_env)
                }
            }
        }
        _ => Err(Error::EvalError(
            "the first element of the list should be a symbol".to_string(),
        )),
    }
}

fn eval_do(nodes: &[Object], rc_env: &Rc<RefCell<Option<Environment>>>) -> Result<Object, Error> {
    if nodes.len() == 0 {
        return Err(Error::EvalError(
            "sub-expressions are required in DO expression".to_string(),
        ));
    }

    let child_env = Environment::new(rc_env);
    let rc_child_env = child_env.to_rc_env();

    let mut result = Err(Error::EvalError("unreachable".to_string()));

    for node in nodes {
        result = eval(node, &rc_child_env);
    }

    result
}

fn eval_let(nodes: &[Object], rc_env: &Rc<RefCell<Option<Environment>>>) -> Result<Object, Error> {
    if nodes.len() != 2 {
        return Err(Error::EvalError(
            "expected 2 sub-expressions for the LET expression".to_string(),
        ));
    }

    let name_object = &nodes[0];

    match name_object {
        Object::Symbol(name) => {
            let value_object = eval(&nodes[1], rc_env)?;
            rc_env_define(rc_env, name, value_object.clone())?;
            Ok(value_object)
        }
        _ => Err(Error::EvalError(
            "the identifier should be a string/symbol".to_string(),
        )),
    }
}

fn eval_if(nodes: &[Object], rc_env: &Rc<RefCell<Option<Environment>>>) -> Result<Object, Error> {
    // e.g. (if test sequence alternative)

    if nodes.len() != 3 {
        return Err(Error::EvalError(
            "expected 3 sub-expressions for the IF expression".to_string(),
        ));
    }

    let test_object = eval(&nodes[0], rc_env)?;
    match test_object {
        Object::Bool(b) => {
            if b {
                eval(&nodes[1], rc_env)
            } else {
                eval(&nodes[2], rc_env)
            }
        }
        _ => Err(Error::EvalError(
            "expected a bool value for the IF test expression".to_string(),
        )),
    }
}

fn eval_defn(nodes: &[Object], rc_env: &Rc<RefCell<Option<Environment>>>) -> Result<Object, Error> {
    // e.g. (defn name (param1 param2) body)
    if nodes.len() != 3 {
        return Err(Error::EvalError(
            "expected 3 sub-expressions for the DEFN expression".to_string(),
        ));
    }

    let r_name = match &nodes[0] {
        Object::Symbol(name) => Ok(name),
        _ => Err(Error::EvalError(
            "function name should be a symbol".to_string(),
        )),
    }?;

    let params = match &nodes[1] {
        Object::List(list) => {
            let symbol_list: Vec<String> = list
                .iter()
                .filter_map(|x| match x {
                    Object::Symbol(s) => Some(s.clone()),
                    _ => None,
                })
                .collect();

            if symbol_list.len() != list.len() {
                return Err(Error::EvalError(
                    "parameter name should be a string/symbol".to_string(),
                ));
            } else {
                Ok(symbol_list)
            }
        }
        _ => Err(Error::EvalError("expected parameter name list".to_string())),
    }?;

    let body = (&nodes[2]).clone();

    let defn = Object::Function(Box::new(Func::UserDefined(
        r_name.clone(),
        params,
        body,
        Rc::downgrade(rc_env),
    )));

    rc_env_define(rc_env, r_name, defn.clone())?;
    Ok(defn)
}

fn eval_fn(nodes: &[Object], rc_env: &Rc<RefCell<Option<Environment>>>) -> Result<Object, Error> {
    // e.g. (fn (param1 param2) body)
    if nodes.len() != 2 {
        return Err(Error::EvalError(
            "expected 2 sub-expressions for the FN expression".to_string(),
        ));
    }

    let params = match &nodes[0] {
        Object::List(list) => {
            let symbol_list: Vec<String> = list
                .iter()
                .filter_map(|x| match x {
                    Object::Symbol(s) => Some(s.clone()),
                    _ => None,
                })
                .collect();

            if symbol_list.len() != list.len() {
                return Err(Error::EvalError(
                    "parameter name should be a string/symbol".to_string(),
                ));
            } else {
                Ok(symbol_list)
            }
        }
        _ => Err(Error::EvalError("expected parameter name list".to_string())),
    }?;

    let body = (&nodes[1]).clone();

    let defn = Object::Function(Box::new(Func::Closure(params, body, Rc::clone(rc_env))));

    Ok(defn)
}

fn eval_function_call(
    node: &Object,
    rest_nodes: &[Object],
    rc_env: &Rc<RefCell<Option<Environment>>>,
) -> Result<Object, Error> {
    let first_eval = eval(node, rc_env)?;
    match first_eval {
        Object::Function(f) => match f.as_ref() {
            Func::Builtin(bf) => {
                let args = rest_nodes
                    .iter()
                    .map(|n| eval(n, rc_env))
                    .collect::<Result<Vec<Object>, Error>>()?;

                bf(&args)
            }
            Func::UserDefined(_, params, body, static_scope_env) => {
                let args = rest_nodes
                    .iter()
                    .map(|n| eval(n, rc_env))
                    .collect::<Result<Vec<Object>, Error>>()?;

                if args.len() != params.len() {
                    return Err(Error::EvalError("args length error".to_string()));
                }

                // 填充实参
                let mut records = HashMap::<String, Object>::new();
                for (idx, name) in params.iter().enumerate() {
                    records.insert(name.clone(), args[idx].clone());
                }

                let option_define_env = static_scope_env.upgrade();
                match option_define_env {
                    Some(define_env) => {
                        let activate_env = Environment::new_with_records(records, &define_env);
                        let rc_activate_env = activate_env.to_rc_env();
                        eval(body, &rc_activate_env)
                    }
                    None => Err(Error::EvalError(
                        "static scope environment not found.".to_string(),
                    )),
                }
            }
            Func::Closure(params, body, static_scope_env) => {
                let args = rest_nodes
                    .iter()
                    .map(|n| eval(n, rc_env))
                    .collect::<Result<Vec<Object>, Error>>()?;

                if args.len() != params.len() {
                    return Err(Error::EvalError("args length error".to_string()));
                }

                // 填充实参
                let mut records = HashMap::<String, Object>::new();
                for (idx, name) in params.iter().enumerate() {
                    records.insert(name.clone(), args[idx].clone());
                }

                // 注：这里跟 Func::UserDefined 的不同
                let activate_env = Environment::new_with_records(records, static_scope_env);
                let rc_activate_env = activate_env.to_rc_env();
                eval(body, &rc_activate_env)
            }
        },
        _ => Err(Error::EvalError("expected a function".to_string())),
    }
}

// 解析一个字符串
pub fn eval_from_string(
    program: &str,
    rc_env: &Rc<RefCell<Option<Environment>>>,
) -> Result<Object, Error> {
    let tokens = tokenize(program);
    let (object, rest_tokens) = parse(&tokens)?;

    if rest_tokens.len() > 0 {
        return Err(Error::EvalError("invalid expression".to_string()));
    }

    let value = eval(&object, rc_env)?;

    Ok(value)
}
