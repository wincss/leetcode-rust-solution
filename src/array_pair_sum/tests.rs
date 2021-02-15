use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
}
