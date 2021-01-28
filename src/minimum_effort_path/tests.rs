use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]),
        1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::minimum_effort_path(vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1]
        ]),
        0
    );
}
