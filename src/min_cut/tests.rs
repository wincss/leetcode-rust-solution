use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_cut("aab".to_string()), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_cut("a".to_string()), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_cut("ab".to_string()), 1);
}
