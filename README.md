# (Practice) Toy LISP - Rust

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [(Practice) Toy LISP - Rust](#practice-toy-lisp-rust)
  - [使用方法](#使用方法)
    - [测试](#测试)
    - [进入 REPL 模式（交互模式）](#进入-repl-模式交互模式)
    - [运行指定的脚本](#运行指定的脚本)
  - [程序示例](#程序示例)
    - [斐波那契数列](#斐波那契数列)
    - [匿名函数和闭包](#匿名函数和闭包)
  - [语法](#语法)
    - [基本数据类型](#基本数据类型)
    - [基本表达式](#基本表达式)
    - [内置函数](#内置函数)

<!-- /code_chunk_output -->

练习使用 Rust lang 编写简单的 _玩具 LISP_ 解析器。

> 注：本项目是学习 Rust 的随手练习，并无实际用途。有关 LISP 的实现原理可以参考 《Build Your Own Lisp》 https://buildyourownlisp.com/

## 使用方法

### 测试

`$ cargo test`

### 进入 REPL 模式（交互模式）

`$ cargo run -- --repl`

### 运行指定的脚本

`$ cargo run -- path_to_script_file`

例如

`$ cargo run -- example/01-add.cjs`

如无意外，应该能看到输出 `7`。

## 程序示例

### 斐波那契数列

0、1、1、2、3、5、8、13、21、34、55...

```clojure
(do
    (defn fib (a)
        (if
            (lte a 1)
            a
            (add
                (fib (sub a 1))
                (fib (sub a 2))
            )
        )
    )
    (fib 10)
)
```

程序运行结果应该是 `55`。

### 匿名函数和闭包

```clojure
(do
    (defn inc_x
        (x)
        (fn
            (i)
            (add x i)
        )
    )
    (let inc_two (inc_x 2))
    (inc_two 10)
)
```

程序运行结果应该是 `12`。

## 语法

### 基本数据类型

只支持整型（int64）和布尔型（字面量为 `true` 和 `false`）两种数据。整型和布尔型被严格区分，不支持隠式转换。比如 `条件分支表达式` 要求 `测试子表达式` 的值必须为布尔型，另外 `逻辑与或非` 运算也要求参数必须是布尔型的数据。

### 基本表达式

- `do` 执行一组表达式，返回最后一个表达式的值；
- `let` 在当前的作用域内绑定一个值，返回被绑定的值；
- `if` 条件分支表达式；
- `defn` 用户自定义函数的定义；
- `fn` 匿名函数的定义。

`匿名函数` 其实也是 `用户自定义函数`，两者不同的是：

1. `用户自定义函数` 本身带有函数名称，而 `匿名函数` 没有名称，不过 `匿名函数` 可以通过 `let` 表达式让它绑定到一个标识符；
2. `用户自定义函数` 不捕获动态产生的作用域的标识符的值。

之所以区分 `defn` 以及 `fn` 主要是为了试验 Rust 的 Weak 和 Rc 两者的区别。

### 内置函数

- `add` 加
- `sub` 减
- `mul` 乘
- `div` 除
- `gt` 大于
- `gte` 大于等于
- `lt` 小于
- `lte` 小于等于
- `eq` 等于
- `neq` 不等于
- `and` 逻辑与
- `or` 逻辑或
- `not` 逻辑非
