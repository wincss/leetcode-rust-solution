use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::sum_odd_length_subarrays(vv![1, 4, 2, 5, 3]), 58);
}

#[test]
fn case_2() {
    assert_eq!(Solution::sum_odd_length_subarrays(vv![1, 2]), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::sum_odd_length_subarrays(vv![10, 11, 12]), 66);
}
