use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}
