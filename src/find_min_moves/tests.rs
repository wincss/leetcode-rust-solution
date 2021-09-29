use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_min_moves(vec![1, 0, 5]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_min_moves(vec![0, 3, 0]), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::find_min_moves(vec![0, 2, 0]), -1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::find_min_moves(vec![0, 0, 11, 5]), 8);
}
