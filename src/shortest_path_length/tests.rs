use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]),
        4
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::shortest_path_length(vec![
            vec![1],
            vec![0, 2, 4],
            vec![1, 3, 4],
            vec![2],
            vec![1, 2]
        ]),
        4
    );
}
