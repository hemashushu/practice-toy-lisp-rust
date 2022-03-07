use crate::ast::Object;
use crate::environment::Environment;
use crate::error::Error;

fn tokenize(expr: &str) -> Vec<String> {
    // 这里用了偷懒的方法来分词
    let tokens = expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
    tokens
}

// 解析一系列 tokens，返回 ast::Object 和剩余的 tokens
fn parse(tokens: &[String]) -> Result<(Object, &[String]), Error> {
    let (token, rest_tokens) = tokens
        .split_first()
        .ok_or(Error("required at least one token".to_string()))?;

    match token.as_str() {
        "(" => parse_list(rest_tokens),
        ")" => Err(Error("unexpected right paren".to_string())),
        _ => Ok((parse_single_object(token), rest_tokens)),
    }
}

// 解析列表，返回 ast::Object::List 和剩余的 tokens
fn parse_list(tokens: &[String]) -> Result<(Object, &[String]), Error> {
    let mut objects: Vec<Object> = vec![];
    let mut remain_tokens = tokens;

    loop {
        let (token, rest_tokens) = remain_tokens
            .split_first()
            .ok_or(Error("missing right paren".to_string()))?;

        if token == ")" {
            return Ok((Object::List(objects), rest_tokens));
        }

        let (object, new_remain_tokens) = parse(&remain_tokens)?;
        objects.push(object);
        remain_tokens = new_remain_tokens;
    }
}

// 解析单独一个 atom，返回 ast::Object
fn parse_single_object(token: &String) -> Object {
    match token.as_str() {
        "true" => Object::Bool(true),
        "false" => Object::Bool(false),
        _ => {
            let maybe_number = token.parse::<i64>();
            match maybe_number {
                Ok(i) => Object::Number(i),
                _ => Object::Symbol(token.clone()),
            }
        }
    }
}

fn eval(node: &Object, env: &mut Environment) -> Result<Object, Error> {
    match node {
        // 标识符，从 Environment 里获取对应的值
        Object::Symbol(name) => env.lookup(name),
        Object::Number(_) => Ok(node.clone()),
        Object::Bool(_) => Ok(node.clone()),
        Object::List(list) => {
            let (first_node, rest_nodes) =
                list.split_first().ok_or(Error("empty list".to_string()))?;
            eval_list(first_node, rest_nodes, env)
        }
        _ => Err(Error("unsupported object".to_string())),
    }
}

fn eval_list(node: &Object, rest_nodes: &[Object], env: &mut Environment) -> Result<Object, Error> {
    match node {
        Object::Symbol(name) => {
            // 先判断是否关键字，比如 do, let, if, fn 等
            match name.as_str() {
                "do" => eval_do(rest_nodes, env),
                "let" => eval_let(rest_nodes, env),
                "if" => eval_if(rest_nodes, env),
                _ => {
                    // 预期是函数（内置函数或者用户自定义函数）
                    eval_function(node, rest_nodes, env)
                }
            }
        }
        _ => Err(Error(
            "expected the first form of the list is a symbol".to_string(),
        )),
    }
}

fn eval_do(nodes: &[Object], env: &mut Environment) -> Result<Object, Error> {
    Err(Error("not implementd yet".to_string()))
}

fn eval_let(nodes: &[Object], env: &mut Environment) -> Result<Object, Error> {
    Err(Error("not implementd yet".to_string()))
}

fn eval_if(nodes: &[Object], env: &mut Environment) -> Result<Object, Error> {
    // e.g. (if test sequence alternative)

    if nodes.len() != 3 {
        return Err(Error("expected 3 forms for the IF expression".to_string()));
    }

    let test_object = eval(&nodes[0], env)?;
    match test_object {
        Object::Bool(b) => {
            if b {
                eval(&nodes[1], env)
            } else {
                eval(&nodes[2], env)
            }
        }
        _ => Err(Error(
            "expected a bool value for the IF test expression".to_string(),
        )),
    }
}

fn eval_function(
    node: &Object,
    rest_nodes: &[Object],
    env: &mut Environment,
) -> Result<Object, Error> {
    let first_eval = eval(node, env)?;
    match first_eval {
        Object::Func(f) => {
            let args = rest_nodes
                .iter()
                .map(|n| eval(n, env))
                .collect::<Result<Vec<Object>, Error>>();
            f(&args?)
        }
        _ => Err(Error("expected a function".to_string())),
    }
}

// 解析一个字符串
pub fn eval_from_string(program: &str, env: &mut Environment) -> Result<Object, Error> {
    let tokens = tokenize(program);
    let (object, rest_tokens) = parse(&tokens)?;

    if rest_tokens.len() > 0 {
        return Err(Error("invalid expression".to_string()));
    }

    let value = eval(&object, env)?;

    Ok(value)
}