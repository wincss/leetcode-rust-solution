use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::get_maximum_generated(0), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::get_maximum_generated(1), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::get_maximum_generated(2), 1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::get_maximum_generated(3), 2);
}

#[test]
fn case_5() {
    assert_eq!(Solution::get_maximum_generated(7), 3);
}
