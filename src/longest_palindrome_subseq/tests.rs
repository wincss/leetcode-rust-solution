use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::longest_palindrome_subseq(String::from("bbbab")),
        4
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::longest_palindrome_subseq(String::from("cbbd")), 2);
}
