use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_power_of_two(1));
}

#[test]
fn case_2() {
    assert!(Solution::is_power_of_two(16));
}

#[test]
fn case_3() {
    assert!(!Solution::is_power_of_two(218));
}

#[test]
fn case_4() {
    assert!(!Solution::is_power_of_two(0));
}
