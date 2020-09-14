use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::expect_number(vec![1, 2, 3]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::expect_number(vec![1, 1]), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::expect_number(vec![1, 2, 1]), 2);
}
