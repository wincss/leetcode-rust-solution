use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
}

#[test]
fn case_3() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 2]), 1);
}
