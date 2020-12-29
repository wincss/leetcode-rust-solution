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

#[test]
fn case_4() {
    assert_eq!(Solution::min_patches(vec![1, 2, 31, 33], 2147483647), 28);
}
