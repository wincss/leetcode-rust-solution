use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_digit_one(13), 6);
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_digit_one(100), 21);
}

#[test]
fn case_3() {
    assert_eq!(Solution::count_digit_one(1000), 301);
}

#[test]
fn case_4() {
    assert_eq!(Solution::count_digit_one(100000000), 80000001);
}

#[test]
fn case_5() {
    assert_eq!(Solution::number_of2s_in_range(25), 9);
}
