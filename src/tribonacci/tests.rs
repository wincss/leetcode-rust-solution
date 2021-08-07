use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::tribonacci(4), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::tribonacci(25), 1389537);
}
