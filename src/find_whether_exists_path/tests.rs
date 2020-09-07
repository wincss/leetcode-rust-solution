use crate::*;

#[test]
fn case_1() {
    assert!(Solution::find_whether_exists_path(
        3,
        vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![1, 2]],
        0,
        2
    ));
}

#[test]
fn case_2() {
    assert!(Solution::find_whether_exists_path(
        5,
        vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 4],
            vec![0, 4],
            vec![0, 1],
            vec![1, 3],
            vec![1, 4],
            vec![1, 3],
            vec![2, 3],
            vec![3, 4]
        ],
        0,
        4
    ));
}

#[test]
fn case_3() {
    assert!(!Solution::find_whether_exists_path(
        12,
        vec![
            vec![0, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 10],
            vec![1, 11],
            vec![1, 4],
            vec![2, 4],
            vec![2, 6],
            vec![2, 9],
            vec![2, 10],
            vec![2, 4],
            vec![2, 5],
            vec![2, 10],
            vec![3, 7],
            vec![3, 7],
            vec![4, 5],
            vec![4, 11],
            vec![4, 11],
            vec![4, 10],
            vec![5, 7],
            vec![5, 10],
            vec![6, 8],
            vec![7, 11],
            vec![8, 10]
        ],
        2,
        3
    ));
}
