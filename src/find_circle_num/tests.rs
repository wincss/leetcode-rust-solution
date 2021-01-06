use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
        3
    );
}
