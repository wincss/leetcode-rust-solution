use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]));
}

#[test]
fn case_2() {
    assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
}

#[test]
fn case_3() {
    assert!(!Solution::is_possible(vec![1, 2, 3, 4, 4, 5]));
}
