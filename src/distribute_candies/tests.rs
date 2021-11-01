use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::distribute_candies(vv![1, 1, 2, 2, 3, 3]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::distribute_candies(vv![1, 1, 2, 3]), 2);
}
