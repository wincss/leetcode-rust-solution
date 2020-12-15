use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::monotone_increasing_digits(10), 9);
}

#[test]
fn case_2() {
    assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
}

#[test]
fn case_3() {
    assert_eq!(Solution::monotone_increasing_digits(332), 299);
}
