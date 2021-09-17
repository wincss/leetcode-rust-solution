use crate::*;

#[test]
fn case_1() {
    assert!(!Solution::can_win_nim(4));
}

#[test]
fn case_2() {
    assert!(Solution::can_win_nim(1));
}
