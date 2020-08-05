use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::length_of_longest_substring("bbbbb".to_string()),
        1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_string()),
        3
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::length_of_longest_substring("tmmzuxt".to_string()),
        5
    )
}
