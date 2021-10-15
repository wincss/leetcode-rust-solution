use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_and_say(1), s!("1"));
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_and_say(2), s!("11"));
}

#[test]
fn case_3() {
    assert_eq!(Solution::count_and_say(3), s!("21"));
}

#[test]
fn case_4() {
    assert_eq!(Solution::count_and_say(4), s!("1211"));
}

#[test]
fn case_5() {
    assert_eq!(Solution::count_and_say(5), s!("111221"));
}
