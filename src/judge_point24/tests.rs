use crate::*;

#[test]
fn case_1() {
    assert!(Solution::judge_point24(vec![4, 1, 8, 7]));
}

#[test]
fn case_2() {
    assert!(!Solution::judge_point24(vec![1, 2, 1, 2]));
}

#[test]
fn case_3() {
    assert!(Solution::judge_point24(vec![3, 3, 8, 8]));
}

#[test]
fn case_4() {
    assert!(Solution::judge_point24(vec![3, 1, 5, 1]));
}
