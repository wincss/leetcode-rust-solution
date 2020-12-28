use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
}
