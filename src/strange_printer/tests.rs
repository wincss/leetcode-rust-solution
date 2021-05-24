use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::strange_printer("aaabbb".to_string()), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::strange_printer("aba".to_string()), 2);
}
