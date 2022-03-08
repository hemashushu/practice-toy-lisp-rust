use crate::error::Error;
use core::fmt;

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
    // name, params, body
    //
    // todo::
    // Closure 本应该还有一个 &Environment 成员，用于记录函数定义时的环境
    // 不过暂时还不知道如何用 rust 实现，所以目前 closure 没有闭包功能
    Closure(String, Vec<String>, Object), // &Environment
}

// 实现 Display trait 能自动获得 ToString，
// 所以不需要单独实现 ToString trait。
impl fmt::Display for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Object::Symbol(s) => s.clone(),     // 标识符和关键字以字符串原样返回
            Object::Number(n) => n.to_string(), // 数字转换为字符串返回
            Object::Bool(b) => b.to_string(),   // 布尔型转为字符串返回
            Object::List(list) => {
                let ss: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", ss.join(" "))
            }
            Object::Function(f) => match f.as_ref() {
                Func::Builtin(_) => "fn".to_string(),
                Func::Closure(name, params, body) => {
                    format!("(defn {} ({}) {})", name, params.join(" "), body)
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
