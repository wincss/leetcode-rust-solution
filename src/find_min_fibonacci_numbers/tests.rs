use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::find_min_fibonacci_numbers(19), 3);
}
