use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_isomorphic(
        String::from("egg"),
        String::from("add")
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::is_isomorphic(
        String::from("foo"),
        String::from("bar")
    ));
}

#[test]
fn case_3() {
    assert!(Solution::is_isomorphic(
        String::from("paper"),
        String::from("title")
    ));
}

#[test]
fn case_4() {
    assert!(!Solution::is_isomorphic(
        String::from("paper"),
        String::from("aaaaa")
    ));
}
