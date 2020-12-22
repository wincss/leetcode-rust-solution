use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
}

#[test]
fn case_3() {
    assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
}

#[test]
fn case_4() {
    assert_eq!(Solution::max_sliding_window(vec![9, 11], 2), vec![11]);
}

#[test]
fn case_5() {
    assert_eq!(Solution::max_sliding_window(vec![4, -2], 2), vec![4]);
}
