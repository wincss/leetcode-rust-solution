use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::bitwise_complement(5), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::bitwise_complement(7), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::bitwise_complement(10), 5);
}

#[test]
fn case_4() {
    assert_eq!(Solution::bitwise_complement(0), 1);
}
