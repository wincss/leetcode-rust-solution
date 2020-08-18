use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_substrings("aba".to_string()), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}
