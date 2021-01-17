use crate::*;

#[test]
fn case_1() {
    assert!(Solution::check_straight_line(vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![6, 7]
    ]));
}

#[test]
fn case_2() {
    assert!(!Solution::check_straight_line(vec![
        vec![1, 1],
        vec![2, 2],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![7, 7]
    ]));
}
