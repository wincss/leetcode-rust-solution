use crate::*;

#[test]
fn case_1() {
    assert!(Solution::backspace_compare(
        String::from("ab#c"),
        String::from("ad#c")
    ));
}

#[test]
fn case_2() {
    assert!(Solution::backspace_compare(
        String::from("ab##"),
        String::from("c#d#")
    ));
}
#[test]
fn case_3() {
    assert!(Solution::backspace_compare(
        String::from("a##c"),
        String::from("#a#c")
    ));
}

#[test]
fn case_4() {
    assert!(!Solution::backspace_compare(
        String::from("a#c"),
        String::from("b")
    ));
}
