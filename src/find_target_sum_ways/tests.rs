use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
}
