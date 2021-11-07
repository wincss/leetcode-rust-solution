use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::get_hint(s!("1807"), s!("7810")), s!("1A3B"));
}

#[test]
fn case_2() {
    assert_eq!(Solution::get_hint(s!("1123"), s!("0111")), s!("1A1B"));
}

#[test]
fn case_3() {
    assert_eq!(Solution::get_hint(s!("1"), s!("0")), s!("0A0B"));
}

#[test]
fn case_4() {
    assert_eq!(Solution::get_hint(s!("1"), s!("1")), s!("1A0B"));
}
