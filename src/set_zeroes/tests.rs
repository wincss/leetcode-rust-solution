use crate::*;

#[test]
fn case_1() {
    let mut input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut input);
    assert_eq!(input, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
}

#[test]
fn case_2() {
    let mut input = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut input);
    assert_eq!(
        input,
        vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
    );
}
