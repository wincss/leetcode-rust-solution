use crate::*;

#[test]
fn case_1() {
    assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1))
}

#[test]
fn case_2() {
    assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2))
}

#[test]
fn case_3() {
    assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 0], 2))
}

#[test]
fn case_4() {
    assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 0], 3))
}
#[test]
fn case_5() {
    assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2));
}

#[test]
fn case_6() {
    assert!(!Solution::can_place_flowers(
        vec![1, 0, 0, 0, 0, 0, 0, 0],
        4
    ));
}

#[test]
fn case_7() {
    assert!(Solution::can_place_flowers(vec![0, 0, 1, 0, 1], 1));
}
