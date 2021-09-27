use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::num_decodings_91("12".to_string()), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::num_decodings_91("226".to_string()), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::num_decodings_91("0".to_string()), 0);
}

#[test]
fn case_4() {
    assert_eq!(Solution::num_decodings_91("06".to_string()), 0);
}

#[test]
fn case_5() {
    assert_eq!(Solution::num_decodings_91("1".to_string()), 1);
}

#[test]
fn case_6() {
    assert_eq!(Solution::num_decodings_91("11".to_string()), 2);
}

#[test]
fn case_639_1() {
    assert_eq!(Solution::num_decodings_639(s!("*")), 9);
}

#[test]
fn case_639_2() {
    assert_eq!(Solution::num_decodings_639(s!("1*")), 18);
}

#[test]
fn case_639_3() {
    assert_eq!(Solution::num_decodings_639(s!("2*")), 15);
}

#[test]
fn case_639_4() {
    assert_eq!(Solution::num_decodings_639(s!("2*5")), 17);
}

#[test]
fn case_639_5() {
    assert_eq!(Solution::num_decodings_639(s!("2*5*8")), 170);
}

#[test]
fn case_639_6() {
    assert_eq!(Solution::num_decodings_639(s!("**")), 96);
}
#[test]
fn case_639_7() {
    assert_eq!(Solution::num_decodings_639(s!("*********")), 291868912);
}
