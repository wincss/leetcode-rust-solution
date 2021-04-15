use super::solution::*;

#[test]
fn case_1() {
    let mut v = Trie::new();
    v.insert("apple".to_string());
    assert!(v.search("apple".to_string()));
    assert!(!v.search("app".to_string()));
    assert!(v.starts_with("app".to_string()));
    v.insert("app".to_string());
    assert!(v.search("app".to_string()));
}
