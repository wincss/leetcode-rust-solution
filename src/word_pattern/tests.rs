use crate::*;

#[test]
fn case_1() {
    assert!(Solution::word_pattern(
        String::from("abba"),
        String::from("dog cat cat dog")
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::word_pattern(
        String::from("abba"),
        String::from("dog cat cat fish")
    ));
}

#[test]
fn case_3() {
    assert!(!Solution::word_pattern(
        String::from("aaaa"),
        String::from("dog cat cat dog")
    ));
}

#[test]
fn case_4() {
    assert!(!Solution::word_pattern(
        String::from("abba"),
        String::from("dog dog dog dog")
    ));
}

#[test]
fn case_5() {
    assert!(!Solution::word_pattern(
        String::from("aaa"),
        String::from("dog dog dog dog")
    ));
}
