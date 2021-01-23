use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_length_of_lcis(vec![]), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
}
