use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::maximum_gap(vec![10]), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::maximum_gap(vec![1, 10000000]), 9999999);
}

#[test]
fn case_4() {
    assert_eq!(Solution::maximum_gap(vec![1, 1, 1, 1]), 0);
}
