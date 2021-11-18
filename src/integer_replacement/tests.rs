use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::integer_replacement(8), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::integer_replacement(7), 4);
}

#[test]
fn case_3() {
    assert_eq!(Solution::integer_replacement(4), 2);
}

#[test]
fn case_4() {
    assert_eq!(Solution::integer_replacement(65535), 17);
}
