#[test]
fn dictionary_test1() {
    use super::super::dictionary::Dictionary;

    let mut dic: Dictionary<i32> = Dictionary::new();
    dic.add_scope("scope1");

    dic.add("a", &0);

    dic.add_scope("scope2");

    dic.add("b", &1);

    assert_eq!(Some(&0), dic.find("a"));
    assert_eq!(Some(&1), dic.find("b"));

    dic.remove_scope(); // Remove scope2

    assert_eq!(Some(&0), dic.find("a"));
    assert_eq!(None, dic.find("b"));

    dic.remove_scope(); // Remove scope1

    assert_eq!(None, dic.find("a"));
    assert_eq!(None, dic.find("b"));
}
