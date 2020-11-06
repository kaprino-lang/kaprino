#[test]
fn test_term_object1() {
    use super::super::term_object;

    let parsed = term_object::term_parser("2 ^ 3 * 5 / 4").unwrap();
    assert_eq!(parsed.1.eval(), 10);
}

#[test]
fn test_term_object2() {
    use super::super::term_object;

    let parsed = term_object::term_parser("45").unwrap();
    assert_eq!(parsed.1.eval(), 45);
}
