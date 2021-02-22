use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::longest_common_subsequence(String::from("abcde"), String::from("ace")),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::longest_common_subsequence(String::from("abc"), String::from("abc")),
        3
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::longest_common_subsequence(String::from("abc"), String::from("def")),
        0
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::longest_common_subsequence(String::from("ezupkr"), String::from("ubmrapg")),
        2
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::longest_common_subsequence(String::from("bsbininm"), String::from("jmjkbkjkv")),
        1
    );
}
