use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::pivot_index(vec![1, 0]), 0);
}

#[test]
fn case_4() {
    assert_eq!(Solution::pivot_index(vec![-1, 1]), -1);
}
