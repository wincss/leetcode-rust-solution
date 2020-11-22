use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
        4
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
        2
    );
}

#[test]
fn case_4() {
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2]]), 1);
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![2, 3], vec![2, 3]]),
        1
    );
}
