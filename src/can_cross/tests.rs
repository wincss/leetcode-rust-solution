use crate::*;

#[test]
fn case_1() {
    assert!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]));
}

#[test]
fn case_2() {
    assert!(!Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]));
}
