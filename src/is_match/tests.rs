use crate::*;

#[test]
fn case_1() {
    assert!(!Solution::is_match(String::from("aa"), String::from("a")));
}

#[test]
fn case_2() {
    assert!(Solution::is_match(String::from("aa"), String::from("a*")));
}

#[test]
fn case_3() {
    assert!(Solution::is_match(String::from("ab"), String::from(".*")));
}

#[test]
fn case_4() {
    assert!(Solution::is_match(
        String::from("aab"),
        String::from("c*a*b")
    ));
}

#[test]
fn case_5() {
    assert!(!Solution::is_match(
        String::from("mississippi"),
        String::from("mis*is*p*.")
    ));
}

#[test]
fn case_6() {
    assert!(Solution::is_match(String::from("a"), String::from("ab*")));
}
