use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::to_hex(26), s!("1a"));
}

#[test]
fn case_2() {
    assert_eq!(Solution::to_hex(-1), s!("ffffffff"));
}

#[test]
fn case_3() {
    assert_eq!(Solution::to_hex(0), s!("0"));
}
