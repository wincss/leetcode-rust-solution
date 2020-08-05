use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::reverse(123), 321);
}

#[test]
fn case_2() {
    assert_eq!(Solution::reverse(-123), -321);
}

#[test]
fn case_3() {
    assert_eq!(Solution::reverse(120), 21);
}

#[test]
fn case_4() {
    assert_eq!(Solution::reverse(2147483647), 0)
}

#[test]
fn case_5() {
    assert_eq!(Solution::reverse(-2147483648), 0)
}
