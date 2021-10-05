use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::third_max(vv![3, 2, 1]), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::third_max(vv![1, 2]), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::third_max(vv![2, 2, 3, 1]), 1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::third_max(vv![1, 2, 3]), 1);
}
