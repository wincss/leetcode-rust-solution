use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
}

#[test]
fn case_3() {
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
}

#[test]
fn case_4() {
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
}

#[test]
fn case_5() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
