use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
}

#[test]
fn case_2() {
    assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
}

#[test]
fn case_3() {
    assert_eq!(Solution::num_rabbits(vec![]), 0);
}
