use crate::*;

#[test]
fn case_1() {
    assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
}

#[test]
fn case_2() {
    assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
}

#[test]
fn case_3() {
    assert!(Solution::contains_duplicate(vec![
        1, 1, 1, 3, 3, 4, 3, 2, 4, 2
    ]));
}
