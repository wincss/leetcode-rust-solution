use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_valid("()".to_string()));
}

#[test]
fn case_2() {
    assert!(Solution::is_valid("()[]{}".to_string()));
}

#[test]
fn case_3() {
    assert!(!Solution::is_valid("[)".to_string()));
}

#[test]
fn case_4() {
    assert!(!Solution::is_valid("())".to_string()));
}

#[test]
fn case_5() {
    assert!(!Solution::is_valid("(()".to_string()));
}

#[test]
fn case_6() {
    assert!(Solution::is_valid("".to_string()));
}
