use toy_lisp::{ast::Object, eval};

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
