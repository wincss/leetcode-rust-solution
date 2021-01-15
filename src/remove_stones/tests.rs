use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2]
        ]),
        5
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 2],
            vec![1, 1],
            vec![2, 0],
            vec![2, 2]
        ]),
        3
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::remove_stones(vec![vec![0, 0],]), 0);
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::remove_stones(vec![
            vec![5, 9],
            vec![9, 0],
            vec![0, 0],
            vec![7, 0],
            vec![4, 3],
            vec![8, 5],
            vec![5, 8],
            vec![1, 1],
            vec![0, 6],
            vec![7, 5],
            vec![1, 6],
            vec![1, 9],
            vec![9, 4],
            vec![2, 8],
            vec![1, 3],
            vec![4, 2],
            vec![2, 5],
            vec![4, 1],
            vec![0, 2],
            vec![6, 5]
        ]),
        19
    );
}
