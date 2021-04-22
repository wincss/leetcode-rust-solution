use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3), 3);
}
