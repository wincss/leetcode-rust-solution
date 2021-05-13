use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::int_to_roman(3), "III".to_string());
}

#[test]
fn case_2() {
    assert_eq!(Solution::int_to_roman(4), "IV".to_string());
}

#[test]
fn case_3() {
    assert_eq!(Solution::int_to_roman(9), "IX".to_string());
}

#[test]
fn case_4() {
    assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
}

#[test]
fn case_5() {
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
}
