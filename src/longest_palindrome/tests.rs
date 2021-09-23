use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::longest_palindrome(String::from("cacb"), String::from("cbba")),
        5
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::longest_palindrome(String::from("ab"), String::from("ab")),
        3
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::longest_palindrome(String::from("aa"), String::from("bb")),
        0
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::longest_palindrome(String::from("f"), String::from("b")),
        0
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::longest_palindrome(String::from("f"), String::from("fb")),
        2
    );
}
