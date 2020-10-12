use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::num_dup_digits_at_most_n(20), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::num_dup_digits_at_most_n(100), 10);
}

#[test]
fn case_3() {
    assert_eq!(Solution::num_dup_digits_at_most_n(1000), 262);
}
