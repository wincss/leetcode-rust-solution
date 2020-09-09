use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::total_n_queens(4), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::total_n_queens(8), 92);
}

#[test]
fn case_3() {
    assert_eq!(Solution::total_n_queens(2), 0);
}
