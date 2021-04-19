use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_operations(vec![8]), 0);
}
