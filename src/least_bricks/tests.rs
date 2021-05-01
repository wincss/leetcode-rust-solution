use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::least_bricks(vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1]
        ]),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::least_bricks(vec![vec![1], vec![1], vec![1]]), 3);
}
