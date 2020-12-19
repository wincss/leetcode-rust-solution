use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::remove_duplicate_letters(String::from("bcabc")),
        String::from("abc")
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::remove_duplicate_letters(String::from("cbacdcbc")),
        String::from("acdb")
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::remove_duplicate_letters(String::from("abacb")),
        String::from("abc")
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::smallest_subsequence(String::from("cdadabcc")),
        String::from("adbc")
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::smallest_subsequence(String::from("abcd")),
        String::from("abcd")
    );
}

#[test]
fn case_6() {
    assert_eq!(
        Solution::smallest_subsequence(String::from("ecbacba")),
        String::from("eacb")
    );
}

#[test]
fn case_7() {
    assert_eq!(
        Solution::smallest_subsequence(String::from("leetcode")),
        String::from("letcod")
    );
}
