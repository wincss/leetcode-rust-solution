use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
}
