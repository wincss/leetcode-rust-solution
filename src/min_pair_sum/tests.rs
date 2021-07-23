use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
}
