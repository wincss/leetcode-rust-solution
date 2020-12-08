use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::unique_paths(3, 2), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::unique_paths(7, 3), 28);
}

#[test]
fn case_3() {
    assert_eq!(Solution::unique_paths(1, 2), 1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::unique_paths(10, 10), 48620);
}

#[test]
fn case_5() {
    assert_eq!(Solution::unique_paths(51, 9), 1916797311);
}
