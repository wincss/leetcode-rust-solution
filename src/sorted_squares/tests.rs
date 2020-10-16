use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::sorted_squares(vec![1]), vec![1]);
}
