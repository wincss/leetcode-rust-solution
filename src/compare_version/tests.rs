use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::compare_version(s!("1.01"), s!("1.001")), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::compare_version(s!("1.0"), s!("1.0.0")), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::compare_version(s!("0.1"), s!("1.1")), -1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::compare_version(s!("1.0.1"), s!("1")), 1);
}

#[test]
fn case_5() {
    assert_eq!(Solution::compare_version(s!("7.5.2.4"), s!("7.5.3")), -1);
}
