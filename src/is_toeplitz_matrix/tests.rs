use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_toeplitz_matrix(vec![
        vec![1, 2, 3, 4],
        vec![5, 1, 2, 3],
        vec![9, 5, 1, 2]
    ]));
}

#[test]
fn case_2() {
    assert!(!Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2],]));
}
