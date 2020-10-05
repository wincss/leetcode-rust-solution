use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::consecutive_numbers_sum(5), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::consecutive_numbers_sum(9), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::consecutive_numbers_sum(15), 4);
}

#[test]
fn case_4() {
    assert_eq!(Solution::consecutive_numbers_sum(918320483), 4);
}

#[test]
fn case_5() {
    assert_eq!(Solution::consecutive_numbers_sum(1000000000), 10);
}
