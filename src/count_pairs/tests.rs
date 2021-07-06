use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_pairs(vec![1, 3, 5, 7, 9]), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_pairs(vec![1, 1, 1, 3, 3, 3, 7]), 15);
}
