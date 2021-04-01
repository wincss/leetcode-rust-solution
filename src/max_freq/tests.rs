use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::max_freq("aababcaab".to_string(), 2, 3, 4), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_freq("aaaa".to_string(), 1, 3, 3), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::max_freq("aabcabcab".to_string(), 2, 2, 3), 3);
}

#[test]
fn case_4() {
    assert_eq!(Solution::max_freq("abcde".to_string(), 2, 3, 3), 0);
}
