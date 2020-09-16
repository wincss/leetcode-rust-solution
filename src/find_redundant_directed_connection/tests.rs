use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        vec![2, 3]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_redundant_directed_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 1],
            vec![1, 5]
        ]),
        vec![4, 1]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::find_redundant_directed_connection(vec![
            vec![1, 2],
            vec![4, 1],
            vec![2, 3],
            vec![3, 4],
        ]),
        vec![3, 4]
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::find_redundant_directed_connection(vec![
            vec![5, 2],
            vec![5, 1],
            vec![3, 1],
            vec![3, 4],
            vec![3, 5]
        ]),
        vec![3, 1]
    );
}
