use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::sort_string(String::from("aaaabbbbcccc")),
        "abccbaabccba"
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::sort_string(String::from("rat")), "art");
}

#[test]
fn case_3() {
    assert_eq!(Solution::sort_string(String::from("leetcode")), "cdelotee");
}

#[test]
fn case_4() {
    assert_eq!(Solution::sort_string(String::from("ggggggg")), "ggggggg");
}

#[test]
fn case_5() {
    assert_eq!(Solution::sort_string(String::from("spo")), "ops");
}
