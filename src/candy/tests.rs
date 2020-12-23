use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
}

#[test]
fn case_2() {
    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
}
