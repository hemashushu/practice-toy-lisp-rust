use crate::{env::Environment, error::Error};
use core::fmt;
use std::{cell::RefCell, rc::{Weak, Rc}};

// AST 的节点跟求值后数据共用一个枚举类型
// Environment 的记录也是共用这个枚举类型
#[derive(Clone)]
pub enum Object {
    Symbol(String),      // 标识符（identifier）或者关键字（如 if, let, fn 等）
    Bool(bool),          // 布尔型
    Number(i64),         // 整数
    List(Vec<Object>),   // 子列表
    Function(Box<Func>), // 函数
}

#[derive(Clone)]
pub enum Func {
    // 内置函数
    // fn (param: &[Object]) -> Result<Object, Error> {...}
    Builtin(fn(&[Object]) -> Result<Object, Error>),

    // 用户自定义函数
    // name, params, body, static scope environment
    UserDefined(
        String,
        Vec<String>,
        Object,

        // 用户自定义函数无法绑定动态产生的作用域，比如在 defn 里面定义 defn 并返回该函数，
        // 该函数离开外层的 defn 之后，随着外层的 defn 的作用域结束，该函数所绑定的
        // 作用域（即之前所捕获的值）也随之消失。
        //
        // 这里之所以区分 defn 和 fn，主要是为了试验 Weak 和 Rc 的区别。
        Weak<RefCell<Option<Environment>>>,
    ),

    // 匿名函数
    // params, body, static scope environment
    Closure(Vec<String>, Object, Rc<RefCell<Option<Environment>>>),
}

// 实现 Display trait 能自动获得 ToString，
// 所以不需要单独实现 ToString trait。
impl fmt::Display for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Object::Symbol(s) => s.clone(),     // 标识符和关键字以字符串原样返回
            Object::Number(n) => n.to_string(), // 数字转换为字符串返回
            Object::Bool(b) => b.to_string(),   // 布尔型转为字符串返回
            Object::List(l) => {
                let ss: Vec<String> = l.iter().map(|x| x.to_string()).collect();
                format!("({})", ss.join(" "))
            }
            Object::Function(f) => match f.as_ref() {
                Func::Builtin(_) => "(builtin)".to_string(),
                Func::UserDefined(name, params, body, _) => {
                    format!("(defn {} ({}) {})", name, params.join(" "), body)
                },
                Func::Closure(params, body, _) => {
                    format!("(fn ({}) {})", params.join(" "), body)
                }
            },
        };

        write!(formatter, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Object;

    #[test]
    fn test_number_to_string() {
        let v1 = Object::Number(123);
        assert_eq!(v1.to_string().as_str(), "123");
    }

    #[test]
    fn test_symbol_to_string() {
        let v1 = Object::Symbol("foo".to_string());
        assert_eq!(v1.to_string().as_str(), "foo");
    }

    #[test]
    fn test_list_to_string() {
        let v1 = Object::Symbol("foo".to_string());
        let v2 = Object::Number(123);
        let v3 = Object::Bool(true);

        let v = Object::List(vec![v1, v2, v3]);
        assert_eq!(v.to_string().as_str(), "(foo 123 true)");
    }
}
