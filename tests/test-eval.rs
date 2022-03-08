use toy_lisp::{ast::Func, ast::Object, eval};

#[test]
fn eval_base_expression() {
    let r1 = eval("(add 1 2)").expect("eval failed");
    assert!(matches!(r1, Object::Number(3)));
}

#[test]
fn eval_nested_expression() {
    let r1 = eval(
        "\
        (add 1 (mul 2 3))",
    )
    .expect("eval failed");

    assert!(matches!(r1, Object::Number(7)));
}

#[test]
fn eval_if() {
    let r1 = eval("(if true 1 2)").expect("eval failed");
    assert!(matches!(r1, Object::Number(1)));

    let r2 = eval("(if false 1 2)").expect("eval failed");
    assert!(matches!(r2, Object::Number(2)));
}

#[test]
fn eval_do() {
    let r1 = eval("(do 1 2 3)").expect("eval failed");
    assert!(matches!(r1, Object::Number(3)));

    let r2 = eval(
        "\
        (do (add 1 2) (mul 2 3))",
    )
    .expect("eval failed");
    assert!(matches!(r2, Object::Number(6)));
}

#[test]
fn eval_let() {
    let r1 = eval(
        "\
        (let foo 1)
        ",
    )
    .expect("eval failed");
    assert!(matches!(r1, Object::Number(1)));

    let r2 = eval(
        "\
        (do
            (let foo 2)
            foo
        )
        ",
    )
    .expect("eval failed");
    assert!(matches!(r2, Object::Number(2)));

    let r3 = eval(
        "\
        (do
            (let foo 1)
            (do
                (let foo 2)
                foo
            )
        )
        ",
    )
    .expect("eval failed");
    assert!(matches!(r3, Object::Number(2)));

    let r3 = eval(
        "\
        (do
            (let foo 1)
            (do
                (let foo 2)
            )
            foo
        )
        ",
    )
    .expect("eval failed");
    assert!(matches!(r3, Object::Number(1)));
}

#[test]
fn eval_defn() {
    let r1 = eval(
        "\
        (do
            (defn name (a b) (add a b))
            name
        )
        ",
    )
    .expect("eval failed");

    match &r1 {
        Object::Function(f) => {
            let c = f.as_ref();
            matches!(*c, Func::Closure(..));
        }
        _ => assert!(false),
    }

    assert_eq!("(defn name (a b) (add a b))", r1.to_string());
}

#[test]
fn eval_defn_call() {
    let r1 = eval(
        "\
        (do
            (defn myadd (a b) (add a b))
            (myadd 1 2)
        )
        ",
    )
    .expect("eval failed");

    assert!(matches!(r1, Object::Number(3)));
}

#[test]
fn eval_fib() {
    let r1 = eval(
        "\
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
        ",
    )
    .expect("eval failed");

    assert!(matches!(r1, Object::Number(55)));
}