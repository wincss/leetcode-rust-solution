use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::num_decodings("226".to_string()), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::num_decodings("0".to_string()), 0);
}

#[test]
fn case_4() {
    assert_eq!(Solution::num_decodings("06".to_string()), 0);
}

#[test]
fn case_5() {
    assert_eq!(Solution::num_decodings("1".to_string()), 1);
}

#[test]
fn case_6() {
    assert_eq!(Solution::num_decodings("11".to_string()), 2);
}
