use crate::*;

#[test]
fn case_1() {
    assert!(Solution::detect_capital_use(s!("USA")));
}

#[test]
fn case_2() {
    assert!(!Solution::detect_capital_use(s!("FlaG")));
}
