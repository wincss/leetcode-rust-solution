use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::range_bitwise_and(0, 1), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::range_bitwise_and(1, 3), 0);
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::range_bitwise_and(2147483646, 2147483647),
        2147483646
    );
}
