use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::minimum_operations("rrryyyrryyyrr".to_string()), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::minimum_operations("ryr".to_string()), 0);
}
