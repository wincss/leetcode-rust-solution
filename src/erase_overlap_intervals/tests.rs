use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
        2
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
        0
    );
}

#[test]
fn case_4() {
    assert_eq!(Solution::erase_overlap_intervals(vec![]), 0);
}
