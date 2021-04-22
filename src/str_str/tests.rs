use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::str_str("aaaaa".to_string(), "bba".to_string()),
        -1
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
}
