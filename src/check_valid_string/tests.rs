use crate::*;

#[test]
fn case_1() {
    assert!(Solution::check_valid_string(s!("()")));
}

#[test]
fn case_2() {
    assert!(Solution::check_valid_string(s!("(*)")));
}

#[test]
fn case_3() {
    assert!(Solution::check_valid_string(s!("(*))")));
}

#[test]
fn case_4() {
    assert!(!Solution::check_valid_string(s!("(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())")));
}
