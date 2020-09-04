use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::get_permutation(3, 3), String::from("213"));
}

#[test]
fn case_2() {
    assert_eq!(Solution::get_permutation(4, 9), String::from("2314"));
}

#[test]
fn case_3() {
    assert_eq!(Solution::get_permutation(5, 80), String::from("42153"));
}
