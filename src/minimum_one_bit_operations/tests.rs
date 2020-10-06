use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::minimum_one_bit_operations(0), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::minimum_one_bit_operations(3), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::minimum_one_bit_operations(6), 4);
}

#[test]
fn case_4() {
    assert_eq!(Solution::minimum_one_bit_operations(9), 14);
}

#[test]
fn case_5() {
    assert_eq!(Solution::minimum_one_bit_operations(333), 393);
}
