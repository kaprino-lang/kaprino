#[test]
fn jit_execute_expr_test1() {
    use super::execute_expr::execute_expr;

    assert_eq!(execute_expr("(6 - 2) * 3 + 5"), Ok((6 - 2) * 3 + 5));
}

#[test]
fn jit_execute_expr_test2() {
    use super::execute_expr::execute_expr;

    assert_eq!(execute_expr("6 - 2 * 3 + 5"), Ok(6 - 2 * 3 + 5));
}

#[test]
fn jit_execute_statement_test1() {
    use super::execute_statement::execute_statement;

    assert_eq!(execute_statement("#ret (6 - 2) * 3 + 5"), Ok((6 - 2) * 3 + 5));
}

#[test]
fn jit_execute_function_test1() {
    use super::execute_function::execute_function;

    assert_eq!(execute_function("f(x) = 35 (Z -> Z)", "f", 0), Ok(35));
}

#[test]
fn jit_execute_function_test2() {
    use super::execute_function::execute_function;

    assert_eq!(execute_function("f(x) = x * x + 10 (Z -> Z)", "f", 5), Ok(5 * 5 + 10));
}

#[test]
fn jit_execute_function_test3() {
    use super::execute_function::execute_function;

    assert_eq!(execute_function("#func f(x) (Z -> Z) |> #ret x * x + 10 |<", "f", 5), Ok(5 * 5 + 10));
}
