# (Practice) Toy LISP - Rust

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [(Practice) Toy LISP - Rust](#practice-toy-lisp-rust)
  - [使用方法](#使用方法)
    - [测试](#测试)
    - [进入 REPL 模式（交互模式）](#进入-repl-模式交互模式)
    - [运行指定的脚本](#运行指定的脚本)
  - [程序示例](#程序示例)
    - [斐波那契数](#斐波那契数)

<!-- /code_chunk_output -->

练习使用 Rust lang 编写简单的 _玩具 LISP_。

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

### 斐波那契数

```clojure
;; 0、1、1、2、3、5、8、13、21、34、55...

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
