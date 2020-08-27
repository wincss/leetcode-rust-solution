use crate::*;

#[test]
fn case_1() {
    assert!(Solution::judge_circle(String::from("UD")));
}

#[test]
fn case_2() {
    assert!(!Solution::judge_circle(String::from("LL")));
}
