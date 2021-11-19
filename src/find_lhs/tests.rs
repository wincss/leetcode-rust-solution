use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_lhs(vv![1, 3, 2, 2, 5, 2, 3, 7]), 5);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_lhs(vv![1, 2, 3, 4]), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::find_lhs(vv![1, 1, 1, 1]), 0);
}
