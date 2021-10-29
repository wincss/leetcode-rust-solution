use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_self_crossing(vv![2, 1, 1, 2]));
}

#[test]
fn case_2() {
    assert!(!Solution::is_self_crossing(vv![1, 2, 3, 4]));
}

#[test]
fn case_3() {
    assert!(Solution::is_self_crossing(vv![1, 1, 1, 1]));
}

#[test]
fn case_4() {
    assert!(Solution::is_self_crossing(vv![1, 1, 2, 1, 1]));
}
