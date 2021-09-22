use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_power_of_three(27));
}

#[test]
fn case_2() {
    assert!(!Solution::is_power_of_three(0));
}

#[test]
fn case_3() {
    assert!(Solution::is_power_of_three(9));
}

#[test]
fn case_4() {
    assert!(!Solution::is_power_of_three(45));
}

#[test]
fn case_5() {
    assert!(Solution::is_power_of_three(1));
}
