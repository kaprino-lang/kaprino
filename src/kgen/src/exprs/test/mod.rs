pub mod jit_compile;

#[test]
fn test_expr_object1() {
    use jit_compile::jit_expr;

    assert_eq!(jit_expr("(6 - 2) * 3 + 5"), (6 - 2) * 3 + 5);
}

#[test]
fn test_expr_object2() {
    use jit_compile::jit_expr;

    assert_eq!(jit_expr("6 - 2 * 3 + 5"), 6 - 2 * 3 + 5);
}
