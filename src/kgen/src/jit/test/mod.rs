#[test]
fn jit_execute_expr_test1() {
    use super::execute_expr::execute_expr;

    assert_eq!(execute_expr("(6 - 2) * 3 + 5").unwrap(), (6 - 2) * 3 + 5);
}

#[test]
fn jit_execute_expr_test2() {
    use super::execute_expr::execute_expr;

    assert_eq!(execute_expr("6 - 2 * 3 + 5").unwrap(), 6 - 2 * 3 + 5);
}

#[test]
fn jit_execute_statement_test1() {
    use super::execute_statement::execute_statement;

    assert_eq!(execute_statement("#ret (6 - 2) * 3 + 5").unwrap(), (6 - 2) * 3 + 5);
}
