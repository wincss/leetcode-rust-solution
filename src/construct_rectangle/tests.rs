use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::construct_rectangle(4), vv![2, 2]);
}

#[test]
fn case_2() {
    assert_eq!(Solution::construct_rectangle(37), vv![37, 1]);
}

#[test]
fn case_3() {
    assert_eq!(Solution::construct_rectangle(122122), vv![427, 286]);
}
