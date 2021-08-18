use crate::*;

#[test]
fn case_1() {
    assert!(Solution::backspace_compare(
        s!("ab#c"),
        s!("ad#c")
    ));
}

#[test]
fn case_2() {
    assert!(Solution::backspace_compare(
        s!("ab##"),
        s!("c#d#")
    ));
}
#[test]
fn case_3() {
    assert!(Solution::backspace_compare(
        s!("a##c"),
        s!("#a#c")
    ));
}

#[test]
fn case_4() {
    assert!(!Solution::backspace_compare(
        s!("a#c"),
        s!("b")
    ));
}
