use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]),
        -1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
        5
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![3, 2, 4], vec![1, 5, 0]]),
        14
    );
}
