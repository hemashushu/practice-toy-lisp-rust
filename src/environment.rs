use std::collections::HashMap;

use crate::ast::Object;
use crate::error::Error;

pub struct Environment<'a> {
    records: HashMap<String, Object>,
    parent: Option<&'a Environment<'a>>,
}

impl Environment<'_> {
    pub fn new<'a>(parent: &'a Environment) -> Environment<'a> {
        let mut records: HashMap<String, Object> = HashMap::new();
        Environment {
            records,
            parent: Some(parent),
        }
    }

    pub fn new_global<'a>() -> Environment<'a> {
        let mut records: HashMap<String, Object> = HashMap::new();

        records.insert("add".to_string(), Object::Func(builtin_fn_add));
        records.insert("sub".to_string(), Object::Func(builtin_fn_sub));
        records.insert("mul".to_string(), Object::Func(builtin_fn_mul));
        records.insert("div".to_string(), Object::Func(builtin_fn_div));

        records.insert("gt".to_string(), Object::Func(builtin_fn_greater_than));
        records.insert(
            "gte".to_string(),
            Object::Func(builtin_fn_greater_or_equal_to),
        );
        records.insert("lt".to_string(), Object::Func(builtin_fn_less_than));
        records.insert("lte".to_string(), Object::Func(builtin_fn_less_or_equal_to));
        records.insert("eq".to_string(), Object::Func(builtin_fn_equal_to));
        records.insert("neq".to_string(), Object::Func(builtin_fn_not_equal_to));

        records.insert("and".to_string(), Object::Func(builtin_fn_and));
        records.insert("or".to_string(), Object::Func(builtin_fn_or));
        records.insert("not".to_string(), Object::Func(builtin_fn_not));

        Environment {
            records,
            parent: None,
        }
    }

    pub fn define(&mut self, name: &str, obj: Object) -> Result<Object, Error> {
        if self.records.contains_key(name) {
            return Err(Error("identifier already exists".to_string()));
        }

        let ns = name.to_string();
        self.records.insert(ns, obj.clone());
        Ok(obj)
    }

    pub fn lookup(&self, name: &str) -> Result<Object, Error> {
        match self.records.get(name) {
            Some(o) => Ok(o.clone()),
            None => match self.parent {
                Some(e) => e.lookup(name),
                _ => Err(Error("identifier not found".to_string())),
            },
        }
    }
}

fn builtin_fn_add(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_number_pair(objs)?;
    Ok(Object::Number(left + right))
}

fn builtin_fn_sub(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_number_pair(objs)?;
    Ok(Object::Number(left - right))
}

fn builtin_fn_mul(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_number_pair(objs)?;
    Ok(Object::Number(left * right))
}

fn builtin_fn_div(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_number_pair(objs)?;
    Ok(Object::Number(left / right))
}

fn builtin_fn_greater_than(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_number_pair(objs)?;
    Ok(Object::Bool(left > right))
}

fn builtin_fn_greater_or_equal_to(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_number_pair(objs)?;
    Ok(Object::Bool(left >= right))
}

fn builtin_fn_less_than(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_number_pair(objs)?;
    Ok(Object::Bool(left < right))
}

fn builtin_fn_less_or_equal_to(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_number_pair(objs)?;
    Ok(Object::Bool(left <= right))
}

fn builtin_fn_equal_to(objs: &[Object]) -> Result<Object, Error> {
    match parse_number_pair(objs) {
        Ok((left, right)) => Ok(Object::Bool(left == right)),
        _ => match parse_bool_pair(objs) {
            Ok((left, right)) => Ok(Object::Bool(left == right)),
            Err(err) => Err(err),
        },
    }
}

fn builtin_fn_not_equal_to(objs: &[Object]) -> Result<Object, Error> {
    let obj = builtin_fn_equal_to(objs)?;
    match obj {
        Object::Bool(b) => Ok(Object::Bool(!b)),
        _ => Err(Error("unreach".to_string())),
    }
}

fn builtin_fn_and(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_bool_pair(objs)?;
    Ok(Object::Bool(left && right))
}

fn builtin_fn_or(objs: &[Object]) -> Result<Object, Error> {
    let (left, right) = parse_bool_pair(objs)?;
    Ok(Object::Bool(left || right))
}

fn builtin_fn_not(objs: &[Object]) -> Result<Object, Error> {
    if objs.len() != 1 {
        return Err(Error("required 1 arguments".to_string()));
    }

    let b = parse_bool(&objs[0])?;
    Ok(Object::Bool(!b))
}

fn parse_number_pair(objs: &[Object]) -> Result<(i64, i64), Error> {
    if objs.len() != 2 {
        return Err(Error("required 2 arguments".to_string()));
    }

    let left = parse_number(&objs[0])?;
    let right = parse_number(&objs[1])?;
    Ok((left, right))
}

fn parse_number(obj: &Object) -> Result<i64, Error> {
    match obj {
        Object::Number(i) => Ok(*i),
        _ => Err(Error("the object is not a number".to_string())),
    }
}

fn parse_bool_pair(objs: &[Object]) -> Result<(bool, bool), Error> {
    if objs.len() != 2 {
        return Err(Error("required 2 arguments".to_string()));
    }

    let left = parse_bool(&objs[0])?;
    let right = parse_bool(&objs[1])?;
    Ok((left, right))
}

fn parse_bool(obj: &Object) -> Result<bool, Error> {
    match obj {
        Object::Bool(b) => Ok(*b),
        _ => Err(Error("the object is not a boolean".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::ast::Object;

    use super::Environment;

    #[test]
    fn test_global_env_builtin_func() {
        let env = Environment::new_global();

        let v1 = env.lookup("add");
        match v1 {
            Ok(f) => {
                assert!(matches!(f, Object::Func(_)));
            }
            _ => assert!(false),
        };
    }

    #[test]
    fn test_define_and_lookup() {
        let mut env = Environment::new_global();

        // 先尝试获取 "foo"，应该返回 Err
        let r1 = env.lookup("foo");
        assert!(matches!(r1, Err(_)));

        // 定义 "foo"，应该返回被定义的对象
        let r2 = env.define("foo", Object::Number(123));
        match r2 {
            Ok(o) => match o {
                Object::Number(n) => assert_eq!(n, 123),
                _ => assert!(false),
            },
            _ => assert!(false),
        };

        // 再次获取 "foo"，应该返回刚被定义的对象
        let r3 = env.lookup("foo");
        match r3 {
            Ok(o) => match o {
                Object::Number(n) => assert_eq!(n, 123),
                _ => assert!(false),
            },
            _ => assert!(false),
        }

        // 再次定义 "foo"，应该返回 Err
        let r4 = env.define("foo", Object::Number(456));
        match r4 {
            Err(_) => {}
            _ => assert!(false),
        };
    }

    #[test]
    fn test_nested_environment() {
        let mut env_parent = Environment::new_global();

        // 尝试从 parent 获取 "foo"，应该返回 Err
        let r1 = env_parent.lookup("foo");
        assert!(matches!(r1, Err(_)));

        {
            let mut env_child = Environment::new(&env_parent);

            // 尝试从 child 获取 "foo"，应该返回 Err
            let c1 = env_child.lookup("foo");
            assert!(matches!(c1, Err(_)));
        }

        // 在 parent 里定义 "foo"
        env_parent.define("foo", Object::Number(123));

        // 尝试从 parent 获取 parent "foo"，应该返回 123
        let r2 = env_parent.lookup("foo");
        match r2 {
            Ok(o) => match o {
                Object::Number(n) => assert_eq!(n, 123),
                _ => assert!(false),
            },
            _ => assert!(false),
        }

        {
            let mut env_child = Environment::new(&env_parent);

            // 尝试从 child 获取 parent 的 "foo"，应该返回 123
            let c1 = env_child.lookup("foo");
            match c1 {
                Ok(o) => match o {
                    Object::Number(n) => assert_eq!(n, 123),
                    _ => assert!(false),
                },
                _ => assert!(false),
            }

            // 尝试在 child 里覆盖 "foo"
            // 注：当前 Environment 允许覆盖上层同名的标识符的值
            let c2 = env_child.define("foo", Object::Number(456));
            match c2 {
                Ok(o) => match o {
                    Object::Number(n) => assert_eq!(n, 456),
                    _ => assert!(false),
                },
                _ => assert!(false),
            };

            // 尝试从 child 获取 child 的 "foo"，应该返回 456
            let c3 = env_child.lookup("foo");
            match c3 {
                Ok(o) => match o {
                    Object::Number(n) => assert_eq!(n, 456),
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }

        // 尝试从 parent 获取 parent "foo"，应该返回 123
        let r3 = env_parent.lookup("foo");
        match r3 {
            Ok(o) => match o {
                Object::Number(n) => assert_eq!(n, 123),
                _ => assert!(false),
            },
            _ => assert!(false),
        }

        {
            let mut env_child = Environment::new(&env_parent);

            // 尝试在 child 里定义 "bar"
            let c1 = env_child.define("bar", Object::Number(789));
            match c1 {
                Ok(o) => match o {
                    Object::Number(n) => assert_eq!(n, 789),
                    _ => assert!(false),
                },
                _ => assert!(false),
            };

            // 尝试从 child 获取 child 的 "bar"，应该返回 789
            let c3 = env_child.lookup("bar");
            match c3 {
                Ok(o) => match o {
                    Object::Number(n) => assert_eq!(n, 789),
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }

        // 尝试从 parent 获取 child "bar"，应该返回 Err
        let r4 = env_parent.lookup("bar");
        assert!(matches!(r4, Err(_)));
    }
}