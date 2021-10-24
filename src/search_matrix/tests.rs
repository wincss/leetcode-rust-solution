use crate::*;

#[test]
fn case_1() {
    assert!(Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        13
    ));
}

#[test]
fn case_ii_1() {
    assert!(Solution::search_matrix_ii(
        vv![
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30]
        ],
        5
    ));
}

#[test]
fn case_ii_2() {
    assert!(!Solution::search_matrix_ii(
        vv![
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30]
        ],
        20
    ));
}

#[test]
fn case_ii_3() {
    assert!(Solution::search_matrix_ii(
        vv![
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30]
        ],
        8
    ));
}
