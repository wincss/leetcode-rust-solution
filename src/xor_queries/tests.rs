use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::xor_queries(
            vec![1, 3, 4, 8],
            vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
        ),
        vec![2, 7, 14, 8]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::xor_queries(
            vec![4, 8, 2, 10],
            vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]
        ),
        vec![8, 0, 4, 4]
    );
}
