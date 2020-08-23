use crate::*;

#[test]
fn case_1() {
    assert!(Solution::repeated_substring_pattern("abab".to_string()));
}

#[test]
fn case_2() {
    assert!(!Solution::repeated_substring_pattern("aba".to_string()));
}

#[test]
fn case_3() {
    assert!(Solution::repeated_substring_pattern(
        "abcabcabcabc".to_string()
    ));
}

#[test]
fn case_4() {
    assert!(Solution::repeated_substring_pattern("aaa".to_string()));
}
