use crate::*;

#[test]
fn case_1() {
    assert!(Solution::contains_nearby_almost_duplicate(
        vec![1, 2, 3, 1],
        3,
        0
    ));
}

#[test]
fn case_2() {
    assert!(Solution::contains_nearby_almost_duplicate(
        vec![1, 0, 1, 1],
        1,
        2
    ));
}

#[test]
fn case_3() {
    assert!(!Solution::contains_nearby_almost_duplicate(
        vec![1, 5, 9, 1, 5, 9],
        2,
        3
    ));
}
