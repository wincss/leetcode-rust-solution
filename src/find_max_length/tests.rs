use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
}
