use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_anagram(
        String::from("anagram"),
        String::from("nagaram")
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::is_anagram(
        String::from("rat"),
        String::from("car")
    ));
}
