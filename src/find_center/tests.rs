use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
        1
    );
}
