use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::fib(0), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::fib(1), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::fib(2), 1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::fib(3), 2);
}

#[test]
fn case_5() {
    assert_eq!(Solution::fib(4), 3);
}
