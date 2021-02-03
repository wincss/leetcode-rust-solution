use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::character_replacement(String::from("ABAB"), 2), 4);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::character_replacement(String::from("AABABBA"), 1),
        4
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::character_replacement(String::from("AABABBA"), 0),
        2
    );
}

#[test]
fn case_4() {
    assert_eq!(Solution::character_replacement(String::new(), 2), 0);
}
