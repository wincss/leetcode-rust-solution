use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::equal_substring(String::from("abcd"), String::from("bcdf"), 3),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::equal_substring(String::from("abcd"), String::from("cdef"), 3),
        1
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::equal_substring(String::from("abcd"), String::from("acde"), 0),
        1
    );
}
