use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::rob_ii(vec![2, 3, 2]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::rob_ii(vec![1, 2, 3, 1]), 4);
}

#[test]
fn case_3() {
    assert_eq!(Solution::rob_ii(vec![0]), 0);
}

#[test]
fn case_4() {
    assert_eq!(Solution::rob_ii(vec![2, 1]), 2);
}

#[test]
fn case_5() {
    assert_eq!(Solution::rob_ii(vec![2, 7, 9, 3, 1]), 11);
}
