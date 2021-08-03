use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::triangle_number(vec![0, 0, 0]), 0);
}
