use crate::{ast::Object, error::Error};

// 解析一系列 tokens，返回 ast::Object 和剩余的 tokens
pub fn parse(tokens: &[String]) -> Result<(Object, &[String]), Error> {
    let (token, rest_tokens) = tokens
        .split_first()
        .ok_or(Error::EvalError("required at least one token".to_string()))?;

    match token.as_str() {
        "(" => parse_list(rest_tokens),
        ")" => Err(Error::EvalError("unexpected right paren".to_string())),
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
            .ok_or(Error::EvalError("missing right paren".to_string()))?;

        if token == ")" {
            return Ok((Object::List(objects), rest_tokens));
        }

        let (object, remain_tokens_after_parse) = parse(&remain_tokens)?;
        objects.push(object);

        remain_tokens = remain_tokens_after_parse;
    }
}

// 解析单独一个元素，返回 ast::Object
// 目前单独元素只支持 `整型` 和 `布尔型` 两种
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
