use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::number_of_steps(14), 6);
}

#[test]
fn case_2() {
    assert_eq!(Solution::number_of_steps(8), 4);
}

#[test]
fn case_3() {
    assert_eq!(Solution::number_of_steps(123), 12);
}

#[test]
fn case_4() {
    assert_eq!(Solution::number_of_steps(0), 0);
}
