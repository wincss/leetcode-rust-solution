use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
}

#[test]
fn case_2() {
    assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}
