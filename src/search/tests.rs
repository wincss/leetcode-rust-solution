use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::search(vv![-1, 0, 3, 5, 9, 12], 9), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::search(vv![-1, 0, 3, 5, 9, 12], 2), -1);
}
