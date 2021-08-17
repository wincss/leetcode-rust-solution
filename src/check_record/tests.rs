use crate::*;

#[test]
fn case_1() {
    assert!(Solution::check_record_551("PPALLP".to_string()));
}

#[test]
fn case_2() {
    assert!(!Solution::check_record_551("PPALLL".to_string()));
}

#[test]
fn case_3() {
    assert_eq!(Solution::check_record_552(1), 3);
}

#[test]
fn case_4() {
    assert_eq!(Solution::check_record_552(2), 8);
}

#[test]
fn case_5() {
    assert_eq!(Solution::check_record_552(10101), 183236316);
}

#[test]
fn case_6() {
    assert_eq!(Solution::check_record_552(100000), 749184020);
}
