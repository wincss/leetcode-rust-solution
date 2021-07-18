use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::max_frequency(vec![1, 2, 4], 5), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_frequency(vec![1, 4, 8, 13], 5), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::max_frequency(vec![3, 9, 6], 2), 1);
}
