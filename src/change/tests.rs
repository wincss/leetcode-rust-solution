use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::change(3, vec![2]), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::change(10, vec![10]), 1);
}
