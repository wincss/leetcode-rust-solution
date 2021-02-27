use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::longest_substring(String::from("aaabb"), 3), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::longest_substring(String::from("ababbc"), 2), 5);
}

#[test]
fn case_3() {
    assert_eq!(Solution::longest_substring(String::from("aba"), 2), 0);
}

#[test]
fn case_4() {
    assert_eq!(Solution::longest_substring(String::from("bbaaacbd"), 3), 3);
}
