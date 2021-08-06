use crate::*;

#[test]
fn case_1() {
    assert!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));
}

#[test]
fn case_2() {
    assert!(!Solution::circular_array_loop(vec![-1, 2]));
}

#[test]
fn case_3() {
    assert!(!Solution::circular_array_loop(vec![-2, 1, -1, -2, -2]));
}
