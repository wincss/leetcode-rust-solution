use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::longest_awesome("3242415".to_string()), 5);
}

#[test]
fn case_2() {
    assert_eq!(Solution::longest_awesome("12345678".to_string()), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::longest_awesome("213123".to_string()), 6);
}

#[test]
fn case_4() {
    assert_eq!(Solution::longest_awesome("00".to_string()), 2);
}
