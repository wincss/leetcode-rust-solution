use crate::*;

#[test]
fn case_1() {
    assert!(Solution::check_possibility(vec![4, 2, 3]));
}

#[test]
fn case_2() {
    assert!(!Solution::check_possibility(vec![4, 2, 1]));
}

#[test]
fn case_3() {
    assert!(!Solution::check_possibility(vec![3, 4, 2, 3]));
}

#[test]
fn case_4() {
    assert!(Solution::check_possibility(vec![5, 7, 1, 8]));
}
