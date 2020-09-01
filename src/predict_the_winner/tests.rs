use crate::*;

#[test]
fn case_1() {
    assert!(!Solution::predict_the_winner(vec![1, 5, 2]));
}

#[test]
fn case_2() {
    assert!(Solution::predict_the_winner(vec![1, 5, 233, 7]));
}

#[test]
fn case_3() {
    assert!(Solution::predict_the_winner(vec![0]));
}
