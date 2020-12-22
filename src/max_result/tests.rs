use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
}
