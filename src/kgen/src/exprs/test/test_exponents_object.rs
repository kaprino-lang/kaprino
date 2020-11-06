#[test]
fn test_exponents_object1() {
    use super::super::exponents_object;

    let parsed = exponents_object::exponents_parser("2 ^ 3 ^ 2").unwrap();
    assert_eq!(parsed.1.eval(), 512);
}

#[test]
fn test_exponents_object2() {
    use super::super::exponents_object;

    let parsed = exponents_object::exponents_parser("45").unwrap();
    assert_eq!(parsed.1.eval(), 45);
}
