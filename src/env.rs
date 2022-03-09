use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{Func, Object};
use crate::error::Error;

pub struct Environment {
    records: HashMap<String, Object>,
    // parent: Option<&'a Environment<'a>>,
    parent: Rc<RefCell<Option<Environment>>>,
}

impl Environment {
    pub fn new(parent: &Rc<RefCell<Option<Environment>>>) -> Environment {
        let records: HashMap<String, Object> = HashMap::new();
        Environment {
            records: records,
            parent: Rc::clone(parent),
        }
    }

    pub fn new_with_records(
        records: HashMap<String, Object>,
        parent: &Rc<RefCell<Option<Environment>>>,
    ) -> Environment {
        Environment {
            records: records,
            parent: Rc::clone(parent),
        }
    }

    pub fn new_global() -> Environment {
        let mut records: HashMap<String, Object> = HashMap::new();

        records.insert(
            "add".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_add))),
        );
        records.insert(
            "sub".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_sub))),
        );
        records.insert(
            "mul".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_mul))),
        );
        records.insert(
            "div".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_div))),
        );

        records.insert(
            "gt".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_greater_than))),
        );
        records.insert(
            "gte".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_greater_or_equal_to))),
        );
        records.insert(
            "lt".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_less_than))),
        );
        records.insert(
            "lte".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_less_or_equal_to))),
        );
        records.insert(
            "eq".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_equal_to))),
        );
        records.insert(
            "neq".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_not_equal_to))),
        );

        records.insert(
            "and".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_and))),
        );
        records.insert(
            "or".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_or))),
        );
        records.insert(
            "not".to_string(),
            Object::Function(Box::new(Func::Builtin(builtin_fn_not))),
        );

        Environment {
            records: records,
            parent: Rc::new(RefCell::new(None)),
        }
    }

    pub fn to_rc_env(self) -> Rc<RefCell<Option<Environment>>> {
        Rc::new(RefCell::new(Some(self)))
    }

    // 如果名称在当前 scope 里已经定义，则返回 Err
    pub fn define(&mut self, name: &str, obj: Object) -> Result<(), Error> {
        if self.records.contains_key(name) {
            return Err(Error::EvalError("identifier already exists".to_string()));
        }

        let ns = name.to_string();
        self.records.insert(ns, obj);

        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<Object> {
        match self.records.get(name) {
            Some(o) => Some(o.clone()),
            None => rc_env_lookup(&self.parent, name),
        }
    }
}

pub fn rc_env_lookup(rc_env: &Rc<RefCell<Option<Environment>>>, name: &str) -> Option<Object> {
    match rc_env.borrow().as_ref() {
        Some(env) => env.lookup(name),
        None => None,
    }
}

pub fn rc_env_define(
    rc_env: &Rc<RefCell<Option<Environment>>>,
    name: &str,
    obj: Object,
) -> Result<(), Error> {
    match rc_env.borrow_mut().as_mut() {
        Some(env) => {
            // env.records.insert(name.to_string(), obj);
            env.define(name, obj)
        }
        None => Err(Error::EvalError("no outer environment".to_string())),
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
        _ => Err(Error::EvalError("unreach".to_string())),
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
        return Err(Error::EvalError("required 1 arguments".to_string()));
    }

    let b = parse_bool(&objs[0])?;
    Ok(Object::Bool(!b))
}

fn parse_number_pair(objs: &[Object]) -> Result<(i64, i64), Error> {
    if objs.len() != 2 {
        return Err(Error::EvalError("required 2 arguments".to_string()));
    }

    let left = parse_number(&objs[0])?;
    let right = parse_number(&objs[1])?;
    Ok((left, right))
}

fn parse_number(obj: &Object) -> Result<i64, Error> {
    match obj {
        Object::Number(i) => Ok(*i),
        _ => Err(Error::EvalError("the object is not a number".to_string())),
    }
}

fn parse_bool_pair(objs: &[Object]) -> Result<(bool, bool), Error> {
    if objs.len() != 2 {
        return Err(Error::EvalError("required 2 arguments".to_string()));
    }

    let left = parse_bool(&objs[0])?;
    let right = parse_bool(&objs[1])?;
    Ok((left, right))
}

fn parse_bool(obj: &Object) -> Result<bool, Error> {
    match obj {
        Object::Bool(b) => Ok(*b),
        _ => Err(Error::EvalError("the object is not a boolean".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;
    use crate::{
        ast::{Func, Object},
        env::{rc_env_define, rc_env_lookup},
    };

    #[test]
    fn test_global_env_builtin_func() {
        let env = Environment::new_global();

        let v1 = env.lookup("add");
        match v1 {
            Some(f) => match f {
                Object::Function(ff) => {
                    assert!(matches!(ff.as_ref(), Func::Builtin(_)))
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn test_define_and_lookup() {
        let mut env = Environment::new_global();

        // 先尝试获取 "foo"，应该返回 Err
        let r1 = env.lookup("foo");
        assert!(matches!(r1, None));

        // 定义 "foo"，应该返回 Ok
        let r2 = env.define("foo", Object::Number(123));
        assert!(matches!(r2, Ok(_)));

        // 再次获取 "foo"，应该返回刚被定义的对象
        let r3 = env.lookup("foo");
        match r3 {
            Some(o) => match o {
                Object::Number(n) => assert_eq!(n, 123),
                _ => assert!(false),
            },
            _ => assert!(false),
        }

        // 再次定义 "foo"，应该返回 Err
        let r4 = env.define("foo", Object::Number(456));
        assert!(matches!(r4, Err(_)));
    }

    #[test]
    fn test_nested_environment() {
        let env_parent = Environment::new_global();
        let rc_env_parent = env_parent.to_rc_env();

        // 尝试从 parent 获取 "foo"，应该返回 Err
        let r1 = rc_env_lookup(&rc_env_parent, "foo");
        assert!(matches!(r1, None));

        {
            let env_child = Environment::new(&rc_env_parent);
            let rc_env_child = env_child.to_rc_env();

            // 尝试从 child 获取 "foo"，应该返回 Err
            let c1 = rc_env_lookup(&rc_env_child, "foo");
            assert!(matches!(c1, None));
        }

        // 在 parent 里定义 "foo"
        let r2 = rc_env_define(&rc_env_parent, "foo", Object::Number(123));
        assert!(matches!(r2, Ok(_)));

        // 尝试从 parent 获取 parent "foo"，应该返回 123
        let r3 = rc_env_lookup(&rc_env_parent, "foo");
        match r3 {
            Some(obj) => match obj {
                Object::Number(n) => assert_eq!(n, 123),
                _ => assert!(false),
            },
            _ => assert!(false),
        }

        {
            let env_child = Environment::new(&rc_env_parent);
            let rc_env_child = env_child.to_rc_env();

            // 尝试从 child 获取 parent 的 "foo"，应该返回 123
            let c1 = rc_env_lookup(&rc_env_child, "foo");
            match c1 {
                Some(obj) => match obj {
                    Object::Number(n) => assert_eq!(n, 123),
                    _ => assert!(false),
                },
                _ => assert!(false),
            }

            // 尝试在 child 里覆盖 "foo"
            // 注：当前 Environment 允许覆盖上层同名的标识符的值
            let c2 = rc_env_define(&rc_env_child, "foo", Object::Number(456));
            assert!(matches!(c2, Ok(_)));

            // 尝试从 child 获取 child 的 "foo"，应该返回 456
            let c3 = rc_env_lookup(&rc_env_child, "foo");
            match c3 {
                Some(o) => match o {
                    Object::Number(n) => assert_eq!(n, 456),
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }

        // 尝试从 parent 获取 parent "foo"，其值应该保持不变，仍然返回 123 而不是 456
        let r4 = rc_env_lookup(&rc_env_parent, "foo");
        match r4 {
            Some(obj) => match obj {
                Object::Number(n) => assert_eq!(n, 123),
                _ => assert!(false),
            },
            _ => assert!(false),
        }

        {
            let env_child = Environment::new(&rc_env_parent);
            let rc_env_child = env_child.to_rc_env();

            // 尝试在 child 里定义 "bar"
            let c1 = rc_env_define(&rc_env_child, "bar", Object::Number(789));
            assert!(matches!(c1, Ok(_)));

            // 尝试从 child 获取 child 的 "bar"，应该返回 789
            let c3 = rc_env_lookup(&rc_env_child, "bar");
            match c3 {
                Some(obj) => match obj {
                    Object::Number(n) => assert_eq!(n, 789),
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }

        // 尝试从 parent 获取 child "bar"，应该返回 Err
        let r5 = rc_env_lookup(&rc_env_parent, "bar");
        assert!(matches!(r5, None));
    }
}
