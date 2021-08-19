use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::reverse_str(s!("abcdefg"), 2), s!("bacdfeg"));
}

#[test]
fn case_2() {
    assert_eq!(Solution::reverse_str(s!("abcd"), 2), s!("bacd"));
}

#[test]
fn case_3() {
    assert_eq!(Solution::reverse_str(s!("abcd"), 4), s!("dcba"));
}
