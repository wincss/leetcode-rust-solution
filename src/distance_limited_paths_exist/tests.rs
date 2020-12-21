use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::distance_limited_paths_exist(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]],
            vec![vec![0, 1, 2], vec![0, 2, 5]]
        ),
        vec![false, true]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::distance_limited_paths_exist(
            5,
            vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]],
            vec![vec![0, 4, 14], vec![1, 4, 13]]
        ),
        vec![true, false]
    );
}
