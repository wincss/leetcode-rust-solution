use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_rotate_steps(String::from("godding"), String::from("gd")),
        4
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_rotate_steps(String::from("qejonvldaejfladlsf"), String::from("adsl")),
        13
    );
}
