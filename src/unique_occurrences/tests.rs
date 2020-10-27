use crate::*;

#[test]
fn case_1() {
    assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]))
}

#[test]
fn case_2() {
    assert!(!Solution::unique_occurrences(vec![1, 2]))
}

#[test]
fn case_3() {
    assert!(Solution::unique_occurrences(vec![
        -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
    ]))
}
