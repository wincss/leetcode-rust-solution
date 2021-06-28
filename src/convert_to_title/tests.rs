use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::convert_to_title(1), "A".to_string());
}

#[test]
fn case_2() {
    assert_eq!(Solution::convert_to_title(28), "AB".to_string());
}

#[test]
fn case_3() {
    assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
}
