use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::min_cost(
            vec![0, 0, 0, 0, 0],
            vec![
                vec![1, 10],
                vec![10, 1],
                vec![10, 1],
                vec![1, 10],
                vec![5, 1]
            ],
            5,
            2,
            3
        ),
        9
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::min_cost(
            vec![0, 2, 1, 2, 0],
            vec![
                vec![1, 10],
                vec![10, 1],
                vec![10, 1],
                vec![1, 10],
                vec![5, 1]
            ],
            5,
            2,
            3
        ),
        11
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::min_cost(
            vec![0, 0, 0, 0, 0],
            vec![
                vec![1, 10],
                vec![10, 1],
                vec![1, 10],
                vec![10, 1],
                vec![1, 10]
            ],
            5,
            2,
            5
        ),
        5
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::min_cost(
            vec![3, 1, 2, 3],
            vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1],],
            4,
            3,
            3
        ),
        -1
    );
}
