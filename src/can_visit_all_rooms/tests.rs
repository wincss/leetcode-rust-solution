use crate::*;

#[test]
fn case_1() {
    assert!(Solution::can_visit_all_rooms(vec![
        vec![1],
        vec![2],
        vec![3],
        vec![]
    ]));
}

#[test]
fn case_2() {
    assert!(!Solution::can_visit_all_rooms(vec![
        vec![1, 3],
        vec![3, 0, 1],
        vec![2],
        vec![0]
    ]));
}
