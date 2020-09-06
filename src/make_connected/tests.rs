use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
        1
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::make_connected(
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
        ),
        2
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
        -1
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::make_connected(5, vec![vec![0, 1], vec![0, 2], vec![3, 4], vec![2, 3]]),
        0
    );
}
