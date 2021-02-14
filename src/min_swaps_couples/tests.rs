use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_swaps_couples(vec![0, 2, 1, 3]), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_swaps_couples(vec![3, 2, 0, 1]), 0);
}
