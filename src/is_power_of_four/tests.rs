use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_power_of_four(1));
}

#[test]
fn case_2() {
    assert!(!Solution::is_power_of_four(5));
}

#[test]
fn case_3() {
    assert!(!Solution::is_power_of_four(2));
}

#[test]
fn case_4() {
    assert!(!Solution::is_power_of_four(8));
}

#[test]
fn case_5() {
    assert!(Solution::is_power_of_four(16));
}
