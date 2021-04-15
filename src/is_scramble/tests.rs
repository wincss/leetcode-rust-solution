use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_scramble(
        "great".to_string(),
        "rgeat".to_string()
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::is_scramble(
        "abcde".to_string(),
        "caebd".to_string()
    ));
}

#[test]
fn case_3() {
    assert!(Solution::is_scramble("a".to_string(), "a".to_string()));
}

#[test]
fn case_4() {
    assert!(!Solution::is_scramble(
        "eebaacbcbcadaaedceaaacadccd".to_string(),
        "eadcaacabaddaceacbceaabeccd".to_string()
    ));
}
