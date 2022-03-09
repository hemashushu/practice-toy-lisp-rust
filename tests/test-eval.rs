use toy_lisp::{ast::Func, ast::Object, env::Environment, error::Error, eval::eval_from_string};

fn internal_eval(program: &str) -> Result<Object, Error> {
    let env = Environment::new_global();
    let rc_env = env.to_rc_env();
    eval_from_string(program, &rc_env)
}

#[test]
fn eval_base_expression() {
    let r1 = internal_eval("(add 1 2)").expect("eval failed");
    assert!(matches!(r1, Object::Number(3)));
}

#[test]
fn eval_nested_expression() {
    let r1 = internal_eval(
        "\
        (add 1 (mul 2 3))",
    )
    .expect("eval failed");

    assert!(matches!(r1, Object::Number(7)));
}

#[test]
fn eval_if() {
    let r1 = internal_eval("(if true 1 2)").expect("eval failed");
    assert!(matches!(r1, Object::Number(1)));

    let r2 = internal_eval("(if false 1 2)").expect("eval failed");
    assert!(matches!(r2, Object::Number(2)));
}

#[test]
fn eval_do() {
    let r1 = internal_eval("(do 1 2 3)").expect("eval failed");
    assert!(matches!(r1, Object::Number(3)));

    let r2 = internal_eval(
        "\
        (do (add 1 2) (mul 2 3))",
    )
    .expect("eval failed");
    assert!(matches!(r2, Object::Number(6)));
}

#[test]
fn eval_let() {
    let r1 = internal_eval(
        "\
        (let foo 1)
        ",
    )
    .expect("eval failed");
    assert!(matches!(r1, Object::Number(1)));

    let r2 = internal_eval(
        "\
        (do
            (let foo 2)
            foo
        )
        ",
    )
    .expect("eval failed");
    assert!(matches!(r2, Object::Number(2)));

    let r3 = internal_eval(
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

    let r3 = internal_eval(
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
    let r1 = internal_eval(
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
            assert!(matches!(*c, Func::UserDefined(..)));
        }
        _ => assert!(false),
    }

    assert_eq!("(defn name (a b) (add a b))", r1.to_string());
}

#[test]
fn eval_defn_call() {
    let r1 = internal_eval(
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
fn eval_fn() {
    let r1 = internal_eval(
        "\
        (fn (a b) (add a b))
        ",
    )
    .expect("eval failed");

    match &r1 {
        Object::Function(f) => {
            let c = f.as_ref();
            assert!(matches!(*c, Func::Closure(..)));
        }
        _ => assert!(false),
    }

    assert_eq!("(fn (a b) (add a b))", r1.to_string());
}

#[test]
fn eval_fn_call() {
    let r1 = internal_eval(
        "\
        (do
            (let myadd (fn (a b) (add a b)))
            (myadd 2 3)
        )
        ",
    )
    .expect("eval failed");

    assert!(matches!(r1, Object::Number(5)));
}

#[test]
fn eval_fib() {
    let r1 = internal_eval(
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

#[test]
fn eval_closure() {
    let r1 = internal_eval(
        "\
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
        ",
    )
    .expect("eval failed");

    assert!(matches!(r1, Object::Number(12)));
}