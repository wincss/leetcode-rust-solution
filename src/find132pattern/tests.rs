use crate::*;

#[test]
fn case_1() {
    assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
}

#[test]
fn case_2() {
    assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
}
#[test]
fn case_3() {
    assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
}

#[test]
fn case_4() {
    assert!(!Solution::find132pattern(vec![1, 0, 1, -4, -3]));
}

#[test]
fn case_5() {
    assert!(!Solution::find132pattern(vec![1]));
}

#[test]
fn case_6() {
    assert!(!Solution::find132pattern(vec![1, 2]));
}
