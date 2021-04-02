use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
}
