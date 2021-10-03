use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::fraction_to_decimal(1, 2), s!("0.5"));
}

#[test]
fn case_2() {
    assert_eq!(Solution::fraction_to_decimal(2, 1), s!("2"));
}

#[test]
fn case_3() {
    assert_eq!(Solution::fraction_to_decimal(2, 3), s!("0.(6)"));
}

#[test]
fn case_4() {
    assert_eq!(Solution::fraction_to_decimal(4, 333), s!("0.(012)"));
}

#[test]
fn case_5() {
    assert_eq!(Solution::fraction_to_decimal(1, 5), s!("0.2"));
}

#[test]
fn case_6() {
    assert_eq!(Solution::fraction_to_decimal(0, -5), s!("0"));
}
