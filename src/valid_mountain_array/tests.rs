use crate::*;

#[test]
fn case_1() {
    assert!(!Solution::valid_mountain_array(vec![2, 1]));
}

#[test]
fn case_2() {
    assert!(!Solution::valid_mountain_array(vec![3, 5, 5]));
}

#[test]
fn case_3() {
    assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
}

#[test]
fn case_4() {
    assert!(!Solution::valid_mountain_array(vec![1, 2, 3]));
}

#[test]
fn case_5() {
    assert!(!Solution::valid_mountain_array(vec![3, 2, 1]));
}
