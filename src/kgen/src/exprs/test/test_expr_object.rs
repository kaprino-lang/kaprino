#[test]
fn test_expr_object1() {
    use super::super::expr_object;

    let parsed = expr_object::expr_parser("(5 + 4) * 3 - 3").unwrap();
    assert_eq!(parsed.1.eval(), 24);
}

#[test]
fn test_expr_object2() {
    use super::super::expr_object;

    let parsed = expr_object::expr_parser("5 + 4 * 3 - 3").unwrap();
    assert_eq!(parsed.1.eval(), 14);
}

#[test]
fn test_expr_object3() {
    use super::super::expr_object;

    let parsed = expr_object::expr_parser("4 * 6 + 3 ^ 2 * 5").unwrap();
    assert_eq!(parsed.1.eval(), 69);
}
