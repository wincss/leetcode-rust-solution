use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::max_num_edges_to_remove(
            4,
            vec![
                vec![3, 1, 2],
                vec![3, 2, 3],
                vec![1, 1, 3],
                vec![1, 2, 4],
                vec![1, 1, 2],
                vec![2, 3, 4],
            ],
        ),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::max_num_edges_to_remove(
            4,
            vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]],
        ),
        0
    );
}
#[test]
fn case_3() {
    assert_eq!(
        Solution::max_num_edges_to_remove(4, vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]],),
        -1
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::max_num_edges_to_remove(
            13,
            vec![
                vec![1, 1, 2],
                vec![2, 2, 3],
                vec![2, 3, 4],
                vec![1, 3, 5],
                vec![3, 2, 6],
                vec![2, 3, 7],
                vec![3, 7, 8],
                vec![3, 2, 9],
                vec![2, 4, 10],
                vec![2, 9, 11],
                vec![1, 2, 12],
                vec![3, 4, 13],
                vec![2, 2, 7],
                vec![1, 1, 9],
                vec![1, 2, 13],
                vec![2, 7, 13],
                vec![3, 2, 3],
                vec![1, 7, 10],
                vec![2, 8, 11],
                vec![1, 2, 7],
                vec![2, 1, 9],
                vec![2, 2, 9],
                vec![1, 5, 6],
                vec![2, 4, 9],
                vec![1, 7, 8],
                vec![1, 4, 6],
                vec![1, 4, 9],
                vec![3, 7, 13],
                vec![2, 2, 8],
                vec![2, 2, 6],
                vec![1, 1, 10],
                vec![1, 1, 11],
                vec![2, 5, 10],
                vec![1, 2, 9],
                vec![2, 1, 2],
                vec![1, 3, 4],
                vec![3, 6, 8],
                vec![3, 6, 13],
                vec![1, 3, 8],
                vec![1, 1, 6],
                vec![3, 3, 9],
                vec![1, 2, 3],
                vec![1, 11, 13]
            ]
        ),
        -1
    );
}
