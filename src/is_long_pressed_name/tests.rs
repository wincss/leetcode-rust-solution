use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_long_pressed_name(
        String::from("alex"),
        String::from("aaleex")
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::is_long_pressed_name(
        String::from("saeed"),
        String::from("ssaaedd")
    ));
}

#[test]
fn case_3() {
    assert!(Solution::is_long_pressed_name(
        String::from("leelee"),
        String::from("lleeelee")
    ));
}

#[test]
fn case_4() {
    assert!(Solution::is_long_pressed_name(
        String::from("laiden"),
        String::from("laiden")
    ));
}

#[test]
fn case_5() {
    assert!(!Solution::is_long_pressed_name(
        String::from("alex"),
        String::from("aaleelx")
    ));
}

#[test]
fn case_6() {
    assert!(!Solution::is_long_pressed_name(
        String::from("alex"),
        String::from("alexxr")
    ));
}
