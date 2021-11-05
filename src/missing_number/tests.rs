use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::missing_number(vv![3, 0, 1]), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::missing_number(vv![0, 1]), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::missing_number(vv![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}

#[test]
fn case_4() {
    assert_eq!(Solution::missing_number(vv![0]), 1);
}
