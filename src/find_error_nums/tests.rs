use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
}
