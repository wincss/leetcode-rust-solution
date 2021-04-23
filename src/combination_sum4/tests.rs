use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
}

#[test]
fn case_2() {
    assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
}
