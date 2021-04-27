use crate::*;

#[test]
fn case_1() {
    assert!(!Solution::judge_square_sum(2147482647));
}

#[test]
fn case_2() {
    assert!(Solution::judge_square_sum(0));
}

#[test]
fn case_3() {
    assert!(Solution::judge_square_sum(1));
}

#[test]
fn case_4() {
    assert!(!Solution::judge_square_sum(3));
}
