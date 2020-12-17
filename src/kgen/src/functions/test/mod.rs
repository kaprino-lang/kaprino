#[test]
fn functions_args_object_test1() {
    use super::args_object::args_parser;

    let (_, generated) = args_parser("apple, grape ,banana").unwrap();

    assert_eq!(generated, vec!["apple", "grape", "banana"]);
}

#[test]
fn functions_args_object_test2() {
    use super::args_object::args_parser;

    let (_, generated) = args_parser("").unwrap();
    let expected: Vec<&str> = Vec::new();

    assert_eq!(generated, expected);
}

#[test]
fn functions_function_type_object_test1() {
    use super::function_type_object::function_type_parser;

    let (_, (args, ret)) = function_type_parser("apple, grape ->banana").unwrap();

    assert_eq!(args, vec!["apple", "grape"]);
    assert_eq!(ret, "banana");
}

#[test]
fn functions_function_type_object_test2() {
    use super::function_type_object::function_type_parser;

    let (_, (args, ret)) = function_type_parser("strawberry->").unwrap();

    assert_eq!(args, vec!["strawberry"]);
    assert_eq!(ret, "");
}
