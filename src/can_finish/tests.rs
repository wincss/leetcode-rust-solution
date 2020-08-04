use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
}

#[test]
fn case_2() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
}
