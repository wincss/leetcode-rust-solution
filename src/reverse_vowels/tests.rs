use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::reverse_vowels(s!("hello")), s!("holle"));
}

#[test]
fn case_2() {
    assert_eq!(Solution::reverse_vowels(s!("leetcode")), s!("leotcede"));
}

#[test]
fn case_3() {
    assert_eq!(Solution::reverse_vowels(s!("Aa")), s!("aA"));
}
