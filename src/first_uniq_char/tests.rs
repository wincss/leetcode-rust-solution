use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::first_uniq_char(String::from("leetcode")), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::first_uniq_char(String::from("loveleetcode")), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::first_uniq_char(String::from("dddccdbba")), 8);
}
