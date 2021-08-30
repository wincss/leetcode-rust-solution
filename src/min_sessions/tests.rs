use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_sessions(vv![1, 2, 3], 3), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_sessions(vv![3, 1, 3, 1, 1], 8), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_sessions(vv![1, 2, 3, 4, 5], 15), 1);
}
