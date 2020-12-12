use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 6]), 6);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
        7
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
        2
    );
}

#[test]
fn case_4() {
    assert_eq!(Solution::wiggle_max_length(vec![0, 0]), 1);
}

#[test]
fn case_5() {
    assert_eq!(Solution::wiggle_max_length(vec![0, 0, 0]), 1);
}

#[test]
fn case_6() {
    assert_eq!(Solution::wiggle_max_length(vec![0, 0, 0, 0]), 1);
}

#[test]
fn case_7() {
    assert_eq!(Solution::wiggle_max_length(vec![3, 3, 3, 2, 5]), 3);
}
