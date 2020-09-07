use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::title_to_number(String::from("A")), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::title_to_number(String::from("AB")), 28);
}

#[test]
fn case_3() {
    assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
}
