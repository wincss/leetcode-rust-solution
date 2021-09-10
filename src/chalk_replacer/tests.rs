use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::chalk_replacer(vv![5, 1, 5], 22), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::chalk_replacer(vv![3, 4, 1, 2], 25), 1);
}
