use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
}

#[test]
fn case_2() {
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
}
