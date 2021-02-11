use crate::*;

#[test]
fn case_0() {
    assert_eq!(Solution::get_row(0), vec![1]);
}

#[test]
fn case_1() {
    assert_eq!(Solution::get_row(1), vec![1, 1]);
}

#[test]
fn case_2() {
    assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
}

#[test]
fn case_3() {
    assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
}

#[test]
fn case_4() {
    assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
}
