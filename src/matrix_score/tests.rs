use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::matrix_score(vec![vec![0, 1], vec![1, 1]]), 5);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
        39
    );
}
