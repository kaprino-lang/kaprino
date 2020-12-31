#[test]
fn exprs_expr_object_test1() {
    use super::expr_object::expr_parser;

    assert_eq!(expr_parser("(6 - 2) * 3 + 5").unwrap().1.eval(), (6 - 2) * 3 + 5);
}

#[test]
fn exprs_expr_object_test2() {
    use super::expr_object::expr_parser;

    assert_eq!(expr_parser("6 - 2 * 3 + 5").unwrap().1.eval(), 6 - 2 * 3 + 5);
}

#[test]
fn exprs_expr_object_test3() {
    use super::expr_object::expr_parser;

    assert_eq!(expr_parser("6 - a * 3 + 5").unwrap().1.eval(), 6 - 0 * 3 + 5);
}
