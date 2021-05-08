use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::minimum_time_required(vec![3, 2, 3], 3), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::minimum_time_required(vec![1, 2, 4, 7, 8], 2), 11);
}
