use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_segments(s!("Hello, my name is John")), 5);
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_segments(s!("Hello")), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::count_segments(s!("love live! mu'sic forever")), 4);
}

#[test]
fn case_4() {
    assert_eq!(Solution::count_segments(s!("")), 0);
}
