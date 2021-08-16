use crate::*;

#[test]
fn case_1() {
    assert!(Solution::check_record("PPALLP".to_string()));
}

#[test]
fn case_2() {
    assert!(!Solution::check_record("PPALLL".to_string()));
}
