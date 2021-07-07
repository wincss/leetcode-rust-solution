use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
}
