use crate::*;

#[test]
fn case_1() {
    assert!(Solution::reordered_power_of2(1));
}

#[test]
fn case_2() {
    assert!(!Solution::reordered_power_of2(10));
}

#[test]
fn case_3() {
    assert!(Solution::reordered_power_of2(16));
}

#[test]
fn case_4() {
    assert!(!Solution::reordered_power_of2(24));
}

#[test]
fn case_5() {
    assert!(Solution::reordered_power_of2(46));
}
