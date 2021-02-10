use crate::*;

#[test]
fn case_1() {
    assert!(Solution::check_inclusion(
        String::from("ab"),
        String::from("eidbaooo")
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::check_inclusion(
        String::from("ab"),
        String::from("eidboaoo")
    ));
}
