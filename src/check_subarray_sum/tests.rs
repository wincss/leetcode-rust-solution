use crate::*;

#[test]
fn case_1() {
    assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
}

#[test]
fn case_2() {
    assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
}

#[test]
fn case_3() {
    assert!(!Solution::check_subarray_sum(vec![0], 1));
}

#[test]
fn case_4() {
    assert!(Solution::check_subarray_sum(vec![5, 0, 0, 0], 3));
}
