use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_the_difference(String::from("abcd"), String::from("abcde")),
        'e'
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_the_difference(String::from(""), String::from("y")),
        'y'
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::find_the_difference(String::from("a"), String::from("aa")),
        'a'
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::find_the_difference(String::from("ae"), String::from("aea")),
        'a'
    );
}
